use std::{iter::FromIterator, str::FromStr};

use amethyst::ecs::{Builder, Entity, WorldExt};
use asset_loading::ASSETS_DEFAULT_DIR;
use asset_model::{
    config::AssetSlugBuilder,
    loaded::{AssetId, ItemId, ItemIds},
};
use control_settings_loading::KeyboardUiGen;
use game_input_model::play::{ButtonInputControlled, NormalInputControlled};
use input_reaction_loading::{IrsLoader, IrsLoaderParams};
use input_reaction_model::loaded::{
    InputReaction, InputReactionsSequenceHandle, InputReactionsSequenceHandles,
};
use kinematic_loading::PositionInitsLoader;
use sequence_loading::{
    SequenceEndTransitionsLoader, SequenceIdMapper, WaitSequenceHandlesLoader, WaitSequenceLoader,
};
use sequence_model::{
    config::{Sequence, SequenceNameString, Sequences, Wait},
    loaded::{SequenceEndTransitions, SequenceIdMappings, WaitSequenceHandles},
};
use smallvec::SmallVec;
use sprite_loading::{
    ScaleSequenceHandlesLoader, ScaleSequenceLoader, SpriteRenderSequenceHandlesLoader,
    SpriteRenderSequenceLoader, TintSequenceHandlesLoader, TintSequenceLoader,
};
use sprite_model::{
    config::{Scale, SpriteRef, SpriteSequenceName, Tint},
    loaded::{ScaleSequenceHandles, SpriteRenderSequenceHandles, TintSequenceHandles},
};
use state_registry::StateId;
use ui_model::config::{UiDefinition, UiType};

use crate::{
    AssetLoadingResources, AssetSequenceComponentLoaderUiCharacterSelection,
    AssetSequenceComponentLoaderUiComponents, AssetSequenceComponentLoaderUiControlSettings,
    AssetSequenceComponentLoaderUiForm, AssetSequenceComponentLoaderUiMapSelection,
    AssetSequenceComponentLoaderUiMenu, AssetSequenceComponentLoaderUiSessionLobby,
    DefinitionLoadingResourcesRead, IdMappingResourcesRead, SequenceComponentLoadingResources,
    TextureLoadingResourcesRead,
};

/// Loads sequence components for UI assets.
#[derive(Debug)]
pub struct AssetSequenceComponentLoaderUi;

impl AssetSequenceComponentLoaderUi {
    /// Loads sequence components for UI assets.
    pub fn load(
        asset_loading_resources: &mut AssetLoadingResources<'_>,
        sequence_component_loading_resources: &mut SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
    ) {
        let AssetLoadingResources {
            ref asset_id_mappings,
            ref asset_type_mappings,
            ref loader,
            ..
        } = asset_loading_resources;
        let SequenceComponentLoadingResources {
            definition_loading_resources_read:
                DefinitionLoadingResourcesRead {
                    ref background_definition_assets,
                    ref ui_definition_assets,
                    ref asset_background_definition_handle,
                    ref asset_ui_definition_handle,
                    ..
                },
            id_mapping_resources_read:
                IdMappingResourcesRead {
                    ref asset_sequence_id_mappings_sprite,
                    ..
                },
            texture_loading_resources_read:
                TextureLoadingResourcesRead {
                    ref asset_sprite_sheet_handles,
                    ..
                },
            ref wait_sequence_assets,
            ref sprite_render_sequence_assets,
            ref input_reactions_assets,
            ref input_reactions_sequence_assets,
            ref tint_sequence_assets,
            ref scale_sequence_assets,
            ..
        } = sequence_component_loading_resources;

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");

        let sprite_sheet_handles = asset_sprite_sheet_handles.get(asset_id);

        let wait_sequence_loader = WaitSequenceLoader {
            loader,
            wait_sequence_assets,
        };
        let mut wait_sequence_handles_loader = WaitSequenceHandlesLoader {
            wait_sequence_loader,
        };
        let tint_sequence_loader = TintSequenceLoader {
            loader,
            tint_sequence_assets,
        };
        let tint_sequence_handles_loader = TintSequenceHandlesLoader {
            tint_sequence_loader,
        };
        let scale_sequence_loader = ScaleSequenceLoader {
            loader,
            scale_sequence_assets,
        };
        let scale_sequence_handles_loader = ScaleSequenceHandlesLoader {
            scale_sequence_loader,
        };
        let sprite_render_sequence_loader = SpriteRenderSequenceLoader {
            loader,
            sprite_render_sequence_assets,
        };
        let sprite_render_sequence_handles_loader = SpriteRenderSequenceHandlesLoader {
            sprite_render_sequence_loader,
        };

        // Begin

        let background_definition = asset_background_definition_handle.get(asset_id).and_then(
            |background_definition_handle| {
                background_definition_assets.get(background_definition_handle)
            },
        );

        let mut ui_definition = asset_ui_definition_handle
            .get(asset_id)
            .and_then(|ui_definition_handle| ui_definition_assets.get(ui_definition_handle))
            .cloned(); // Clone so that we don't mutate the actual read data.

        // Keyboard button labels for `ControlSettings` UI
        let keyboard_button_labels = if let Some(UiDefinition {
            ui_type: UiType::ControlSettings(control_settings),
            sequences,
            ..
        }) = ui_definition.as_mut()
        {
            Some(KeyboardUiGen::generate_full(
                &control_settings.keyboard,
                &sequence_component_loading_resources.player_input_configs,
                sequences,
            ))
        } else {
            None
        };

        let ui_definition = ui_definition.as_ref();

        let mut item_ids_all = ItemIds::default();

        // Load item entities object-wise.
        // `BackgroundDefinition`.
        if let Some(background_definition) = background_definition {
            let sequence_id_mappings = SequenceIdMappings::from_iter(
                background_definition.layers.keys().map(|sequence_string| {
                    SequenceNameString::from_str(sequence_string)
                        .expect("Expected `SequenceNameString::from_str` to succeed.")
                }),
            );
            let sequence_id_mappings = &sequence_id_mappings;
            let sequence_end_transitions_loader = SequenceEndTransitionsLoader {
                sequence_id_mappings,
            };

            let position_inits =
                PositionInitsLoader::items_to_datas(background_definition.layers.values());
            let sequence_id_inits = SequenceIdMapper::<SpriteSequenceName>::strings_to_ids(
                sequence_id_mappings,
                asset_slug,
                background_definition.layers.keys(),
            );
            let sequence_end_transitions = sequence_end_transitions_loader
                .items_to_datas(background_definition.layers.values(), asset_slug);
            let wait_sequence_handles = wait_sequence_handles_loader
                .items_to_datas(background_definition.layers.values(), |layer| {
                    layer.sequence.frames.iter()
                });
            let tint_sequence_handles = tint_sequence_handles_loader
                .items_to_datas(background_definition.layers.values(), |layer| {
                    layer.sequence.frames.iter()
                });
            let scale_sequence_handles = scale_sequence_handles_loader
                .items_to_datas(background_definition.layers.values(), |layer| {
                    layer.sequence.frames.iter()
                });
            let sprite_render_sequence_handles = sprite_sheet_handles.map(|sprite_sheet_handles| {
                sprite_render_sequence_handles_loader.items_to_datas(
                    background_definition.layers.values(),
                    |layer| layer.sequence.frames.iter(),
                    sprite_sheet_handles,
                )
            });

            let asset_world = &mut sequence_component_loading_resources.asset_world;
            let mut item_ids = position_inits
                .0
                .into_iter()
                .zip(sequence_id_inits.into_iter())
                .map(|(position_init, sequence_id_init)| {
                    let mut item_entity_builder = asset_world
                        .create_entity()
                        .with(position_init)
                        .with(sequence_id_init)
                        .with(sequence_end_transitions.clone())
                        .with(wait_sequence_handles.clone())
                        .with(tint_sequence_handles.clone())
                        .with(scale_sequence_handles.clone());

                    if let Some(sprite_render_sequence_handles) =
                        sprite_render_sequence_handles.clone()
                    {
                        item_entity_builder =
                            item_entity_builder.with(sprite_render_sequence_handles);
                    }

                    item_entity_builder.build()
                })
                .map(ItemId::new)
                .collect::<Vec<ItemId>>();

            item_ids_all.append(&mut item_ids)
        }

        // `UiDefinition`.
        if let Some(ui_definition) = ui_definition {
            let UiDefinition {
                ui_type, sequences, ..
            } = ui_definition;

            // For loading `InputReactionsSequence`s.
            let irs_loader_params = IrsLoaderParams {
                loader,
                input_reactions_assets,
                input_reactions_sequence_assets,
            };

            let sequence_id_mappings = asset_sequence_id_mappings_sprite
                .get(asset_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Expected `SequenceIdMappings<SpriteSequenceName>` to exist for `{}`.",
                        asset_slug
                    )
                });

            let (
                sequence_end_transitions,
                wait_sequence_handles,
                tint_sequence_handles,
                scale_sequence_handles,
                sprite_render_sequence_handles,
            ) = Self::sequence_components(
                asset_loading_resources,
                sequence_component_loading_resources,
                asset_id,
                sequences,
            );
            let input_reactions_sequence_handles = {
                let input_reactions_sequence_handles = ui_definition
                    .sequences
                    .values()
                    .map(|sequence| {
                        IrsLoader::load(&irs_loader_params, sequence_id_mappings, None, sequence)
                    })
                    .collect::<Vec<InputReactionsSequenceHandle<InputReaction>>>();
                InputReactionsSequenceHandles::new(input_reactions_sequence_handles)
            };

            // `UiButton`s
            let asset_world = &mut sequence_component_loading_resources.asset_world;
            let mut item_ids_button = ui_definition
                .buttons
                .iter()
                .enumerate()
                .flat_map(|(index, ui_button)| {
                    let mut ui_label = ui_button.label.clone();
                    ui_label.position += ui_button.position;
                    let position_init = ui_label.position;
                    let item_entity_text = asset_world
                        .create_entity()
                        .with(position_init)
                        .with(ui_label)
                        .build();

                    let ui_sprite_label = &ui_button.sprite;
                    let position_init = ui_button.position + ui_sprite_label.position;
                    let sequence_id_init = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
                        sequence_id_mappings,
                        asset_slug,
                        &ui_sprite_label.sequence,
                    );
                    let item_entity_sprite = {
                        let mut item_entity_builder = asset_world
                            .create_entity()
                            .with(position_init)
                            .with(sequence_id_init)
                            .with(sequence_end_transitions.clone())
                            .with(wait_sequence_handles.clone())
                            .with(tint_sequence_handles.clone())
                            .with(scale_sequence_handles.clone())
                            .with(input_reactions_sequence_handles.clone())
                            .with(ButtonInputControlled)
                            .with(NormalInputControlled::new(index as u32));

                        if let Some(sprite_render_sequence_handles) =
                            sprite_render_sequence_handles.clone()
                        {
                            item_entity_builder =
                                item_entity_builder.with(sprite_render_sequence_handles);
                        }

                        item_entity_builder.build()
                    };

                    let item_entities =
                        SmallVec::<[Entity; 2]>::from_buf([item_entity_text, item_entity_sprite]);
                    item_entities.into_iter()
                })
                .map(ItemId::new)
                .collect::<Vec<ItemId>>();
            item_ids_all.append(&mut item_ids_button);

            let asset_sequence_component_loader_ui_components =
                AssetSequenceComponentLoaderUiComponents {
                    sequence_end_transitions,
                    wait_sequence_handles,
                    tint_sequence_handles,
                    scale_sequence_handles,
                    input_reactions_sequence_handles,
                    sprite_render_sequence_handles,
                };

            match ui_type {
                UiType::Form(ui_form_items) => {
                    AssetSequenceComponentLoaderUiForm::load(
                        &mut sequence_component_loading_resources.asset_world,
                        asset_slug,
                        sequence_id_mappings,
                        &asset_sequence_component_loader_ui_components,
                        &mut item_ids_all,
                        ui_form_items,
                    );
                }
                UiType::Menu(ui_menu_items) => {
                    AssetSequenceComponentLoaderUiMenu::load(
                        &mut sequence_component_loading_resources.asset_world,
                        asset_slug,
                        sequence_id_mappings,
                        &asset_sequence_component_loader_ui_components,
                        &mut item_ids_all,
                        &sequence_component_loading_resources.player_controllers,
                        ui_menu_items,
                    );
                }
                UiType::CharacterSelection(character_selection_ui) => {
                    AssetSequenceComponentLoaderUiCharacterSelection::load(
                        asset_type_mappings,
                        &mut sequence_component_loading_resources.asset_world,
                        asset_slug,
                        sequence_id_mappings,
                        &asset_sequence_component_loader_ui_components,
                        &mut item_ids_all,
                        character_selection_ui,
                    );
                }
                UiType::MapSelection(map_selection_ui) => {
                    AssetSequenceComponentLoaderUiMapSelection::load(
                        asset_type_mappings,
                        &mut sequence_component_loading_resources.asset_world,
                        asset_slug,
                        sequence_id_mappings,
                        &asset_sequence_component_loader_ui_components,
                        &mut item_ids_all,
                        map_selection_ui,
                    );
                }
                UiType::SessionLobby(session_lobby_ui) => {
                    AssetSequenceComponentLoaderUiSessionLobby::load(
                        &mut sequence_component_loading_resources.asset_world,
                        &mut item_ids_all,
                        session_lobby_ui,
                    );
                }
                UiType::ControlSettings(control_settings) => {
                    let keyboard_button_labels = keyboard_button_labels
                        .as_ref()
                        .expect("Expected `keyboard_button_labels` to exist.");
                    AssetSequenceComponentLoaderUiControlSettings::load(
                        &mut sequence_component_loading_resources.asset_world,
                        asset_slug,
                        sequence_id_mappings,
                        &asset_sequence_component_loader_ui_components,
                        &mut item_ids_all,
                        control_settings,
                        keyboard_button_labels,
                    );
                }
            }

            if ui_definition.display_control_buttons {
                Self::control_buttons_display(
                    asset_loading_resources,
                    sequence_component_loading_resources,
                    asset_id,
                    &mut item_ids_all,
                );
            }
        }

        sequence_component_loading_resources
            .asset_item_ids
            .insert(asset_id, item_ids_all);
    }

    fn sequence_components<Seq, Frame>(
        asset_loading_resources: &AssetLoadingResources<'_>,
        sequence_component_loading_resources: &SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
        sequences: &Sequences<Seq, SpriteSequenceName, Frame>,
    ) -> (
        SequenceEndTransitions,
        WaitSequenceHandles,
        TintSequenceHandles,
        ScaleSequenceHandles,
        Option<SpriteRenderSequenceHandles>,
    )
    where
        Seq: AsRef<Sequence<SpriteSequenceName, Frame>>,
        Frame: AsRef<Wait> + AsRef<SpriteRef> + AsRef<Tint> + AsRef<Scale>,
    {
        let AssetLoadingResources {
            asset_id_mappings,
            loader,
            ..
        } = asset_loading_resources;
        let SequenceComponentLoadingResources {
            id_mapping_resources_read:
                IdMappingResourcesRead {
                    asset_sequence_id_mappings_sprite,
                    ..
                },
            texture_loading_resources_read:
                TextureLoadingResourcesRead {
                    asset_sprite_sheet_handles,
                    ..
                },
            wait_sequence_assets,
            sprite_render_sequence_assets,
            tint_sequence_assets,
            scale_sequence_assets,
            ..
        } = sequence_component_loading_resources;

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");

        let sprite_sheet_handles = asset_sprite_sheet_handles.get(asset_id);

        let wait_sequence_loader = WaitSequenceLoader {
            loader,
            wait_sequence_assets,
        };
        let mut wait_sequence_handles_loader = WaitSequenceHandlesLoader {
            wait_sequence_loader,
        };
        let tint_sequence_loader = TintSequenceLoader {
            loader,
            tint_sequence_assets,
        };
        let tint_sequence_handles_loader = TintSequenceHandlesLoader {
            tint_sequence_loader,
        };
        let scale_sequence_loader = ScaleSequenceLoader {
            loader,
            scale_sequence_assets,
        };
        let scale_sequence_handles_loader = ScaleSequenceHandlesLoader {
            scale_sequence_loader,
        };
        let sprite_render_sequence_loader = SpriteRenderSequenceLoader {
            loader,
            sprite_render_sequence_assets,
        };
        let sprite_render_sequence_handles_loader = SpriteRenderSequenceHandlesLoader {
            sprite_render_sequence_loader,
        };

        let sequence_id_mappings = asset_sequence_id_mappings_sprite
            .get(asset_id)
            .unwrap_or_else(|| {
                panic!(
                    "Expected `SequenceIdMappings<SpriteSequenceName>` to exist for `{}`.",
                    asset_slug
                )
            });
        let sequence_end_transitions_loader = SequenceEndTransitionsLoader {
            sequence_id_mappings,
        };

        // Should we have sequences per `ItemId` instead of per `AssetId`?
        let sequence_end_transitions =
            sequence_end_transitions_loader.items_to_datas(sequences.values(), asset_slug);
        let wait_sequence_handles =
            wait_sequence_handles_loader.items_to_datas(sequences.values(), |seq| {
                AsRef::<Sequence<SpriteSequenceName, Frame>>::as_ref(seq)
                    .frames
                    .iter()
            });
        let tint_sequence_handles =
            tint_sequence_handles_loader.items_to_datas(sequences.values(), |seq| {
                AsRef::<Sequence<SpriteSequenceName, Frame>>::as_ref(seq)
                    .frames
                    .iter()
            });
        let scale_sequence_handles =
            scale_sequence_handles_loader.items_to_datas(sequences.values(), |seq| {
                AsRef::<Sequence<SpriteSequenceName, Frame>>::as_ref(seq)
                    .frames
                    .iter()
            });
        let sprite_render_sequence_handles = sprite_sheet_handles.map(|sprite_sheet_handles| {
            sprite_render_sequence_handles_loader.items_to_datas(
                sequences.values(),
                |seq| {
                    AsRef::<Sequence<SpriteSequenceName, Frame>>::as_ref(seq)
                        .frames
                        .iter()
                },
                sprite_sheet_handles,
            )
        });

        (
            sequence_end_transitions,
            wait_sequence_handles,
            tint_sequence_handles,
            scale_sequence_handles,
            sprite_render_sequence_handles,
        )
    }

    /// Adds `ItemId`s for control buttons display if requested
    fn control_buttons_display(
        asset_loading_resources: &AssetLoadingResources<'_>,
        sequence_component_loading_resources: &mut SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
        item_ids_all: &mut Vec<ItemId>,
    ) {
        let AssetLoadingResources {
            asset_id_mappings,
            loader,
            ..
        } = asset_loading_resources;
        let SequenceComponentLoadingResources {
            definition_loading_resources_read:
                DefinitionLoadingResourcesRead {
                    ref ui_definition_assets,
                    ref asset_ui_definition_handle,
                    ..
                },
            id_mapping_resources_read:
                IdMappingResourcesRead {
                    ref asset_sequence_id_mappings_sprite,
                    ..
                },
            ref player_input_configs,
            ref camera_zoom_dimensions,
            ref input_reactions_assets,
            ref input_reactions_sequence_assets,
            ..
        } = sequence_component_loading_resources;

        // Look up ControlSettings asset for mini control buttons display.
        let asset_slug_control_settings = AssetSlugBuilder::default()
            .namespace(ASSETS_DEFAULT_DIR.to_string())
            .name(StateId::ControlSettings.to_string())
            .build()
            .expect("Expected control settings asset slug to be valid.");
        let asset_id_control_settings = {
            let asset_id_control_settings =
                asset_id_mappings.id(&asset_slug_control_settings).copied();
            if asset_id_control_settings == Some(asset_id) {
                // If the current asset ID is the same as the control_settings `AssetId`, then we
                // return None -- we don't allow displaying the mini control buttons. This also
                // prevents waiting on our own asset ID to be loaded.
                None
            } else {
                asset_id_control_settings
            }
        };

        // Mini control button labels for other UIs
        let control_buttons_display_data =
            if let Some(asset_id_control_settings) = asset_id_control_settings {
                // Look up UiDefinition for the control settings asset
                let mut ui_definition_control_settings = asset_ui_definition_handle
                    .get(asset_id_control_settings)
                    .and_then(|ui_definition_handle| ui_definition_assets.get(ui_definition_handle))
                    .cloned(); // Clone so that we don't mutate the actual read data.

                if let Some(UiDefinition {
                    ui_type: UiType::ControlSettings(control_settings),
                    sequences,
                    ..
                }) = ui_definition_control_settings.as_mut()
                {
                    let control_buttons_display_labels = KeyboardUiGen::generate_mini(
                        &control_settings.keyboard,
                        player_input_configs,
                        **camera_zoom_dimensions,
                        sequences,
                    );

                    let sequence_id_mappings_control_settings = asset_sequence_id_mappings_sprite
                        .get(asset_id_control_settings)
                        .unwrap_or_else(|| {
                            panic!(
                        "Expected `SequenceIdMappings<SpriteSequenceName>` to exist for `{}`.",
                        asset_slug_control_settings
                    )
                        });
                    let (
                        sequence_end_transitions,
                        wait_sequence_handles,
                        tint_sequence_handles,
                        scale_sequence_handles,
                        sprite_render_sequence_handles,
                    ) = Self::sequence_components(
                        asset_loading_resources,
                        sequence_component_loading_resources,
                        asset_id_control_settings,
                        sequences,
                    );
                    // For loading `InputReactionsSequence`s.
                    let irs_loader_params = IrsLoaderParams {
                        loader,
                        input_reactions_assets,
                        input_reactions_sequence_assets,
                    };
                    let input_reactions_sequence_handles = {
                        let input_reactions_sequence_handles = sequences
                            .values()
                            .map(|sequence| {
                                IrsLoader::load(
                                    &irs_loader_params,
                                    sequence_id_mappings_control_settings,
                                    None,
                                    sequence,
                                )
                            })
                            .collect::<Vec<InputReactionsSequenceHandle<InputReaction>>>();
                        InputReactionsSequenceHandles::new(input_reactions_sequence_handles)
                    };
                    let asset_sequence_component_loader_ui_components =
                        AssetSequenceComponentLoaderUiComponents {
                            sequence_end_transitions,
                            wait_sequence_handles,
                            tint_sequence_handles,
                            scale_sequence_handles,
                            sprite_render_sequence_handles,
                            input_reactions_sequence_handles,
                        };

                    Some((
                        asset_id_control_settings,
                        control_buttons_display_labels,
                        asset_sequence_component_loader_ui_components,
                    ))
                } else {
                    None
                }
            } else {
                None
            };

        if let Some((
            asset_id_control_settings,
            control_buttons_display_labels,
            asset_sequence_component_loader_ui_components,
        )) = control_buttons_display_data
        {
            let sequence_id_mappings_control_settings = asset_sequence_id_mappings_sprite
                .get(asset_id_control_settings)
                .unwrap_or_else(|| {
                    panic!(
                        "Expected `SequenceIdMappings<SpriteSequenceName>` to exist for `{}`.",
                        asset_slug_control_settings
                    )
                });

            let control_button_labels =
                control_buttons_display_labels
                    .iter()
                    .flat_map(|player_control_buttons_labels| {
                        player_control_buttons_labels
                            .axes
                            .values()
                            .chain(player_control_buttons_labels.actions.values())
                    });

            let position_inits = PositionInitsLoader::items_to_datas(control_button_labels.clone());
            let sequence_id_inits = SequenceIdMapper::<SpriteSequenceName>::items_to_datas(
                sequence_id_mappings_control_settings,
                &asset_slug_control_settings,
                control_button_labels
                    .map(|control_button_label| &control_button_label.sprite.sequence),
            );

            let asset_world = &mut sequence_component_loading_resources.asset_world;
            let item_ids = position_inits
                .0
                .into_iter()
                .zip(sequence_id_inits.into_iter())
                .map(|(position_init, sequence_id_init)| {
                    let AssetSequenceComponentLoaderUiComponents {
                        sequence_end_transitions,
                        wait_sequence_handles,
                        tint_sequence_handles,
                        scale_sequence_handles,
                        input_reactions_sequence_handles,
                        sprite_render_sequence_handles,
                    } = asset_sequence_component_loader_ui_components.clone();

                    let mut item_entity_builder = asset_world
                        .create_entity()
                        .with(position_init)
                        .with(sequence_id_init)
                        .with(sequence_end_transitions)
                        .with(wait_sequence_handles)
                        .with(tint_sequence_handles)
                        .with(scale_sequence_handles)
                        .with(input_reactions_sequence_handles)
                        .with(ButtonInputControlled);

                    if let Some(sprite_render_sequence_handles) = sprite_render_sequence_handles {
                        item_entity_builder =
                            item_entity_builder.with(sprite_render_sequence_handles);
                    }

                    item_entity_builder.build()
                })
                .map(ItemId::new);
            item_ids_all.extend(item_ids);
        }
    }
}
