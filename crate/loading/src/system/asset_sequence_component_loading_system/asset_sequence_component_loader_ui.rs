use std::{iter::FromIterator, str::FromStr};

use amethyst::ecs::{Builder, Entity, WorldExt};
use asset_model::loaded::{AssetId, ItemId, ItemIds};
use character_selection_ui_model::{
    config::{CharacterSelectionUi, CswLayer, CswLayerName, CswTemplate},
    loaded::{CharacterSelectionWidget, CswPortraits},
    play::CswMain,
};
use control_settings_loading::KeyboardUiGen;
use game_input::{ButtonInputControlled, InputControlled, SharedInputControlled};
use input_reaction_loading::{IrsLoader, IrsLoaderParams};
use input_reaction_model::loaded::{
    InputReaction, InputReactionsSequenceHandle, InputReactionsSequenceHandles,
};
use kinematic_loading::PositionInitsLoader;
use sequence_loading::{
    SequenceEndTransitionsLoader, SequenceIdMapper, WaitSequenceHandlesLoader, WaitSequenceLoader,
};
use sequence_model::{config::SequenceNameString, loaded::SequenceIdMappings};
use smallvec::SmallVec;
use sprite_loading::{
    ScaleSequenceHandlesLoader, ScaleSequenceLoader, SpriteRenderSequenceHandlesLoader,
    SpriteRenderSequenceLoader, TintSequenceHandlesLoader, TintSequenceLoader,
};
use sprite_model::config::SpriteSequenceName;
use ui_menu_item_model::loaded::UiMenuItem;
use ui_model::config::{UiDefinition, UiType};

use crate::{
    AssetLoadingResources, DefinitionLoadingResourcesRead, SequenceComponentLoadingResources,
    TextureLoadingResourcesRead,
};

/// Loads sequence components for map assets.
#[derive(Debug)]
pub struct AssetSequenceComponentLoaderUi;

impl AssetSequenceComponentLoaderUi {
    /// Loads sequence components for map assets.
    pub fn load(
        asset_loading_resources: &mut AssetLoadingResources<'_>,
        SequenceComponentLoadingResources {
            definition_loading_resources_read:
                DefinitionLoadingResourcesRead {
                    background_definition_assets,
                    ui_definition_assets,
                    asset_background_definition_handle,
                    asset_ui_definition_handle,
                    ..
                },
            texture_loading_resources_read:
                TextureLoadingResourcesRead {
                    asset_sprite_sheet_handles,
                    ..
                },
            asset_world,
            asset_item_ids,
            input_config,
            wait_sequence_assets,
            sprite_render_sequence_assets,
            input_reactions_assets,
            input_reactions_sequence_assets,
            tint_sequence_assets,
            scale_sequence_assets,
            ..
        }: &mut SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
    ) {
        let AssetLoadingResources {
            asset_id_mappings,
            loader,
            ..
        } = asset_loading_resources;

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

        let keyboard_button_labels = if let Some(UiDefinition {
            ui_type: UiType::ControlSettings(control_settings),
            sequences,
            ..
        }) = ui_definition.as_mut()
        {
            Some(KeyboardUiGen::generate(
                &control_settings.keyboard,
                &input_config,
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

            let sequence_id_mappings = SequenceIdMappings::from_iter(sequences.keys());
            let sequence_id_mappings = &sequence_id_mappings;
            let sequence_end_transitions_loader = SequenceEndTransitionsLoader {
                sequence_id_mappings,
            };

            // TODO: Sequences per item instead of per asset.
            let sequence_end_transitions =
                sequence_end_transitions_loader.items_to_datas(sequences.values(), asset_slug);
            let wait_sequence_handles = wait_sequence_handles_loader
                .items_to_datas(sequences.values(), |ui_sequence| {
                    ui_sequence.sequence.frames.iter()
                });
            let tint_sequence_handles = tint_sequence_handles_loader
                .items_to_datas(sequences.values(), |ui_sequence| {
                    ui_sequence.sequence.frames.iter()
                });
            let scale_sequence_handles = scale_sequence_handles_loader
                .items_to_datas(sequences.values(), |ui_sequence| {
                    ui_sequence.sequence.frames.iter()
                });
            let sprite_render_sequence_handles = sprite_sheet_handles.map(|sprite_sheet_handles| {
                sprite_render_sequence_handles_loader.items_to_datas(
                    sequences.values(),
                    |ui_sequence| ui_sequence.sequence.frames.iter(),
                    sprite_sheet_handles,
                )
            });
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
            let mut item_ids_button = ui_definition
                .buttons
                .iter()
                .flat_map(|ui_button| {
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
                            .with(SharedInputControlled);

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

            match ui_type {
                UiType::Menu(ui_menu_items_cfg) => {
                    let mut item_ids = ui_menu_items_cfg
                        .iter()
                        .flat_map(|ui_menu_item_cfg| {
                            let mut ui_label = ui_menu_item_cfg.label.clone();
                            ui_label.position += ui_menu_item_cfg.position;
                            let ui_menu_item = UiMenuItem::new(ui_menu_item_cfg.index);
                            let position_init = ui_label.position;
                            let item_entity_text = asset_world
                                .create_entity()
                                .with(position_init)
                                .with(ui_label)
                                .with(ui_menu_item)
                                .build();

                            let ui_sprite_label = &ui_menu_item_cfg.sprite;
                            let position_init =
                                ui_menu_item_cfg.position + ui_sprite_label.position;
                            let sequence_id_init =
                                SequenceIdMapper::<SpriteSequenceName>::item_to_data(
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
                                    .with(input_reactions_sequence_handles.clone());

                                if let Some(sprite_render_sequence_handles) =
                                    sprite_render_sequence_handles.clone()
                                {
                                    item_entity_builder =
                                        item_entity_builder.with(sprite_render_sequence_handles);
                                }

                                item_entity_builder.build()
                            };

                            let item_entities = SmallVec::<[Entity; 2]>::from_buf([
                                item_entity_text,
                                item_entity_sprite,
                            ]);
                            item_entities.into_iter()
                        })
                        .map(ItemId::new)
                        .collect::<Vec<ItemId>>();

                    item_ids_all.append(&mut item_ids)
                }
                UiType::CharacterSelection(character_selection_ui) => {
                    let CharacterSelectionUi {
                        widgets, // Vec<CswDefinition>
                        widget_template:
                            CswTemplate {
                                portraits:
                                    character_selection_ui_model::config::CswPortraits {
                                        join,
                                        random,
                                        select,
                                    },
                                layers, // IndexMap<String, UiSpriteLabel>
                            },
                        characters_available_selector: _,
                    } = character_selection_ui;

                    // Store widget item IDs in `item_ids_all` to be spawned during state ID
                    // updates, but don't store item IDs for widget template layers as those
                    // are instantiated when each widget item ID is attached to an entity.

                    // Layer item IDs
                    let position_inits_widgets =
                        PositionInitsLoader::items_to_datas(widgets.iter());
                    let sequence_id_join = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
                        sequence_id_mappings,
                        asset_slug,
                        join,
                    );
                    let sequence_id_random = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
                        sequence_id_mappings,
                        asset_slug,
                        random,
                    );
                    let sequence_id_select = SequenceIdMapper::<SpriteSequenceName>::item_to_data(
                        sequence_id_mappings,
                        asset_slug,
                        select,
                    );
                    let csw_portraits = CswPortraits {
                        join: sequence_id_join,
                        random: sequence_id_random,
                        select: sequence_id_select,
                    };
                    let item_ids_layers = position_inits_widgets
                        .0
                        .into_iter()
                        .map(|position_init_widget| {
                            let mut position_inits =
                                PositionInitsLoader::items_to_datas(layers.values());
                            position_inits
                                .iter_mut()
                                .for_each(|position_init| *position_init += position_init_widget);
                            let sequence_id_inits =
                                SequenceIdMapper::<SpriteSequenceName>::items_to_datas(
                                    sequence_id_mappings,
                                    asset_slug,
                                    layers.values().map(AsRef::<SequenceNameString<_>>::as_ref),
                                );

                            position_inits
                                .0
                                .into_iter()
                                .zip(sequence_id_inits.into_iter())
                                .zip(layers.keys())
                                .map(|((position_init, sequence_id_init), csw_layer)| {
                                    let mut item_entity_builder = asset_world
                                        .create_entity()
                                        .with(position_init)
                                        .with(sequence_id_init)
                                        .with(sequence_end_transitions.clone())
                                        .with(wait_sequence_handles.clone())
                                        .with(tint_sequence_handles.clone())
                                        .with(scale_sequence_handles.clone())
                                        .with(input_reactions_sequence_handles.clone());

                                    match csw_layer {
                                        CswLayer::Name(CswLayerName::Main) => {
                                            item_entity_builder = item_entity_builder.with(CswMain);
                                        }
                                        CswLayer::Name(CswLayerName::Portrait) => {
                                            item_entity_builder =
                                                item_entity_builder.with(csw_portraits);
                                        }
                                        _ => {}
                                    }

                                    if let Some(sprite_render_sequence_handles) =
                                        sprite_render_sequence_handles.clone()
                                    {
                                        item_entity_builder = item_entity_builder
                                            .with(sprite_render_sequence_handles);
                                    }

                                    item_entity_builder.build()
                                })
                                .map(ItemId::new)
                                .collect::<Vec<ItemId>>()
                        })
                        .collect::<Vec<Vec<ItemId>>>();

                    // Widget item IDs
                    let input_controlleds = {
                        let controller_count = input_config.controller_configs.len();
                        (0..controller_count)
                            .into_iter()
                            .map(InputControlled::new)
                            .collect::<Vec<InputControlled>>()
                    };
                    let mut item_ids_widgets = item_ids_layers
                        .into_iter()
                        .zip(input_controlleds.into_iter())
                        .map(
                            |(layer_item_ids, input_controlled)| CharacterSelectionWidget {
                                layers: layer_item_ids,
                                input_controlled,
                            },
                        )
                        .map(|character_selection_widget| {
                            asset_world
                                .create_entity()
                                .with(character_selection_widget)
                                .build()
                        })
                        .map(ItemId::new)
                        .collect::<Vec<ItemId>>();

                    item_ids_all.append(&mut item_ids_widgets)
                }
                UiType::ControlSettings(control_settings) => {
                    let keyboard_button_labels = keyboard_button_labels
                        .as_ref()
                        .expect("Expected `keyboard_button_labels` to exist.");
                    let position_inits =
                        PositionInitsLoader::items_to_datas(keyboard_button_labels.iter());
                    let sequence_id_inits = SequenceIdMapper::<SpriteSequenceName>::items_to_datas(
                        sequence_id_mappings,
                        asset_slug,
                        keyboard_button_labels
                            .iter()
                            .map(|control_button_label| &control_button_label.sprite.sequence),
                    );

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
                                .with(scale_sequence_handles.clone())
                                .with(input_reactions_sequence_handles.clone())
                                .with(ButtonInputControlled);

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

                    // Create entity for title label.
                    let item_id_title = {
                        let ui_label = control_settings.title.clone();
                        let position_init = ui_label.position;
                        let entity = asset_world
                            .create_entity()
                            .with(position_init)
                            .with(ui_label)
                            .build();
                        ItemId::new(entity)
                    };
                    item_ids.push(item_id_title);

                    item_ids_all.append(&mut item_ids)
                }
            }
        }

        asset_item_ids.insert(asset_id, item_ids_all);
    }
}
