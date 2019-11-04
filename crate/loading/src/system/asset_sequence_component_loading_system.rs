use std::{any::type_name, str::FromStr};

use amethyst::renderer::SpriteRender;
use asset_model::{
    config::{AssetSlug, AssetType},
    loaded::AssetId,
};
use background_model::{
    config::BackgroundDefinition,
    loaded::{AssetBackgroundLayers, BackgroundLayer, BackgroundLayers},
};
use character_loading::{CtsLoader, CtsLoaderParams, CHARACTER_TRANSITIONS_DEFAULT};
use character_model::{
    config::CharacterSequence,
    loaded::{CharacterCtsHandle, CharacterCtsHandles},
};
use energy_model::config::EnergySequence;
use game_mode_selection_model::GameModeIndex;
use loading_model::loaded::LoadStage;
use log::debug;
use map_model::loaded::Margins;
use object_loading::{ObjectLoader, ObjectLoaderParams};
use object_model::loaded::Object;
use object_type::ObjectType;
use sequence_loading::{
    SequenceEndTransitionMapper, SequenceEndTransitionsLoader, WaitSequenceHandlesLoader,
    WaitSequenceLoader,
};
use sequence_loading_spi::SequenceComponentDataLoader;
use sequence_model::{
    config::{SequenceName, SequenceNameString},
    loaded::{AssetSequenceIdMappings, SequenceEndTransitions, WaitSequenceHandles},
};
use sprite_loading::{
    SpritePositionsLoader, SpriteRenderSequenceHandlesLoader, SpriteRenderSequenceLoader,
};
use sprite_model::{
    config::{SpriteFrame, SpritePosition},
    loaded::{SpritePositions, SpriteRenderSequenceHandles},
};
use typename_derive::TypeName;
use ui_menu_item_model::loaded::{UiMenuItem, UiMenuItems};
use ui_model::config::UiType;

use crate::{
    AssetLoadingResources, AssetPartLoader, AssetPartLoadingSystem, DefinitionLoadingResourcesRead,
    IdMappingResourcesRead, SequenceComponentLoadingResources, TextureLoadingResourcesRead,
};

/// Loads asset sequence components.
pub type AssetSequenceComponentLoadingSystem = AssetPartLoadingSystem<AssetSequenceComponentLoader>;

/// `AssetSequenceComponentLoader`.
#[derive(Debug, TypeName)]
pub struct AssetSequenceComponentLoader;

impl AssetSequenceComponentLoader {
    fn load_background_layers<SeqName>(
        asset_sequence_id_mappings: &AssetSequenceIdMappings<SeqName>,
        asset_background_layers: &mut AssetBackgroundLayers,
        asset_slug: &AssetSlug,
        asset_id: AssetId,
        background_definition: &BackgroundDefinition,
    ) where
        SeqName: SequenceName,
    {
        let background_layers = {
            let sequence_id_mappings =
                asset_sequence_id_mappings.get(asset_id).unwrap_or_else(|| {
                    panic!(
                        "Expected `SequenceIdMappings<{}>` to exist for `{}`.",
                        type_name::<SeqName>(),
                        asset_slug
                    )
                });
            let background_layers = background_definition
                .layers
                .keys()
                .map(|sequence_string| {
                    let sequence = SequenceNameString::from_str(sequence_string)
                        .expect("Expected `SequenceNameString::from_str` to succeed.");
                    sequence_id_mappings
                        .id(&sequence)
                        .copied()
                        .unwrap_or_else(|| {
                            panic!(
                                "Expected `SequenceIdMapping` to exist for \
                                 sequence `{}` for asset `{}`.",
                                sequence, asset_slug
                            )
                        })
                })
                .map(BackgroundLayer::new)
                .collect::<Vec<BackgroundLayer>>();

            BackgroundLayers::new(background_layers)
        };
        asset_background_layers.insert(asset_id, background_layers);
    }
}

impl<'s> AssetPartLoader<'s> for AssetSequenceComponentLoader {
    const LOAD_STAGE: LoadStage = LoadStage::SequenceComponentLoading;
    type SystemData = SequenceComponentLoadingResources<'s>;

    fn preprocess(
        AssetLoadingResources {
            asset_id_mappings, ..
        }: &mut AssetLoadingResources<'_>,
        SequenceComponentLoadingResources {
            asset_sequence_end_transitions,
            asset_wait_sequence_handles,
            asset_source_sequence_handles,
            asset_object_acceleration_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_body_sequence_handles,
            asset_interactions_sequence_handles,
            asset_spawns_sequence_handles,
            ..
        }: &mut SequenceComponentLoadingResources<'_>,
    ) {
        let capacity = asset_id_mappings.capacity();
        asset_sequence_end_transitions.set_capacity(capacity);
        asset_wait_sequence_handles.set_capacity(capacity);
        asset_source_sequence_handles.set_capacity(capacity);
        asset_object_acceleration_sequence_handles.set_capacity(capacity);
        asset_sprite_render_sequence_handles.set_capacity(capacity);
        asset_body_sequence_handles.set_capacity(capacity);
        asset_interactions_sequence_handles.set_capacity(capacity);
        asset_spawns_sequence_handles.set_capacity(capacity);
    }

    fn process(
        asset_loading_resources: &mut AssetLoadingResources<'_>,
        SequenceComponentLoadingResources {
            definition_loading_resources_read:
                DefinitionLoadingResourcesRead {
                    character_definition_assets,
                    energy_definition_assets,
                    map_definition_assets,
                    background_definition_assets,
                    ui_definition_assets,
                    asset_character_definition_handle,
                    asset_energy_definition_handle,
                    asset_map_definition_handle,
                    asset_background_definition_handle,
                    asset_ui_definition_handle,
                },
            id_mapping_resources_read:
                IdMappingResourcesRead {
                    asset_sequence_id_mappings_sprite,
                    asset_sequence_id_mappings_character,
                    asset_sequence_id_mappings_energy,
                    asset_sequence_id_mappings_ui,
                    ..
                },
            texture_loading_resources_read:
                TextureLoadingResourcesRead {
                    asset_sprite_sheet_handles,
                    ..
                },
            source_assets,
            body_assets,
            interactions_assets,
            spawns_assets,
            wait_sequence_assets,
            source_sequence_assets,
            object_acceleration_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
            spawns_sequence_assets,
            character_control_transitions_assets,
            character_cts_assets,
            asset_sequence_end_transitions,
            asset_wait_sequence_handles,
            asset_source_sequence_handles,
            asset_object_acceleration_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_body_sequence_handles,
            asset_interactions_sequence_handles,
            asset_spawns_sequence_handles,
            asset_character_cts_handles,
            asset_background_layers,
            asset_sprite_positions,
            asset_map_bounds,
            asset_margins,
            asset_ui_menu_items,
        }: &mut SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
    ) {
        let AssetLoadingResources {
            asset_id_mappings,
            asset_type_mappings,
            loader,
            ..
        } = asset_loading_resources;

        let asset_type = asset_type_mappings
            .get(asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");

        debug!("Loading `{}` sequence components.", asset_slug,);

        let wait_sequence_loader = WaitSequenceLoader {
            loader,
            wait_sequence_assets,
        };
        let mut wait_sequence_handles_loader = WaitSequenceHandlesLoader {
            wait_sequence_loader,
            asset_wait_sequence_handles,
        };
        let sprite_render_sequence_loader = SpriteRenderSequenceLoader {
            loader,
            sprite_render_sequence_assets,
        };
        let mut sprite_render_sequence_handles_loader = SpriteRenderSequenceHandlesLoader {
            sprite_render_sequence_loader,
            asset_sprite_render_sequence_handles,
        };

        let mut sprite_positions_loader = SpritePositionsLoader {
            asset_sprite_positions,
        };
        let sequence_end_transition_mapper_ui = SequenceEndTransitionMapper {
            asset_sequence_id_mappings: asset_sequence_id_mappings_ui,
        };
        let sequence_end_transition_mapper_sprite = SequenceEndTransitionMapper {
            asset_sequence_id_mappings: asset_sequence_id_mappings_sprite,
        };

        let sprite_sheet_handles = asset_sprite_sheet_handles.get(asset_id);
        match asset_type {
            AssetType::Object(object_type) => {
                let sprite_sheet_handles = sprite_sheet_handles
                    .expect("Expected `SpriteSheetHandles` to exist for object.");
                let object_loader_params = ObjectLoaderParams {
                    loader,
                    asset_id_mappings,
                    asset_type_mappings,
                    asset_sequence_id_mappings_character,
                    asset_sequence_id_mappings_energy,
                    wait_sequence_assets,
                    source_assets,
                    source_sequence_assets,
                    object_acceleration_sequence_assets,
                    sprite_render_sequence_assets,
                    body_sequence_assets,
                    interactions_sequence_assets,
                    spawns_sequence_assets,
                    body_assets,
                    interactions_assets,
                    spawns_assets,
                    sprite_sheet_handles,
                };

                let object = match object_type {
                    ObjectType::Character => {
                        let character_definition = asset_character_definition_handle
                            .get(asset_id)
                            .and_then(|character_definition_handle| {
                                character_definition_assets.get(character_definition_handle)
                            })
                            .expect("Expected `CharacterDefinition` to be loaded.");

                        let sequence_id_mappings = asset_sequence_id_mappings_character
                            .get(asset_id)
                            .expect("Expected `SequenceIdMapping` to be loaded.");

                        let cts_loader_params = CtsLoaderParams {
                            loader: &*loader,
                            character_control_transitions_assets:
                                &*character_control_transitions_assets,
                            character_cts_assets: &*character_cts_assets,
                        };

                        let character_cts_handles = {
                            let character_cts_handles = character_definition
                                .object_definition
                                .sequences
                                .iter()
                                .map(|(sequence_id, sequence)| {
                                    let sequence_default = CHARACTER_TRANSITIONS_DEFAULT
                                        .object_definition
                                        .sequences
                                        .get(sequence_id);

                                    CtsLoader::load(
                                        &cts_loader_params,
                                        sequence_id_mappings,
                                        sequence_default,
                                        sequence,
                                    )
                                })
                                .collect::<Vec<CharacterCtsHandle>>();
                            CharacterCtsHandles::new(character_cts_handles)
                        };
                        asset_character_cts_handles.insert(asset_id, character_cts_handles);

                        ObjectLoader::load::<CharacterSequence>(
                            object_loader_params,
                            &character_definition.object_definition,
                        )
                    }
                    ObjectType::Energy => {
                        let energy_definition = asset_energy_definition_handle
                            .get(asset_id)
                            .and_then(|energy_definition_handle| {
                                energy_definition_assets.get(energy_definition_handle)
                            })
                            .expect("Expected `EnergyDefinition` to be loaded.");

                        ObjectLoader::load::<EnergySequence>(
                            object_loader_params,
                            &energy_definition.object_definition,
                        )
                    }
                    ObjectType::TestObject => panic!("`TestObject` loading is not supported."),
                };
                let Object {
                    sequence_end_transitions,
                    wait_sequence_handles,
                    source_sequence_handles,
                    object_acceleration_sequence_handles,
                    sprite_render_sequence_handles,
                    body_sequence_handles,
                    interactions_sequence_handles,
                    spawns_sequence_handles,
                } = object;

                asset_sequence_end_transitions.insert(asset_id, sequence_end_transitions);
                asset_wait_sequence_handles.insert(asset_id, wait_sequence_handles);
                asset_source_sequence_handles.insert(asset_id, source_sequence_handles);
                asset_object_acceleration_sequence_handles
                    .insert(asset_id, object_acceleration_sequence_handles);
                asset_sprite_render_sequence_handles
                    .insert(asset_id, sprite_render_sequence_handles);
                asset_body_sequence_handles.insert(asset_id, body_sequence_handles);
                asset_interactions_sequence_handles.insert(asset_id, interactions_sequence_handles);
                asset_spawns_sequence_handles.insert(asset_id, spawns_sequence_handles);
            }
            AssetType::Map => {
                let map_definition = asset_map_definition_handle
                    .get(asset_id)
                    .and_then(|map_definition_handle| {
                        map_definition_assets.get(map_definition_handle)
                    })
                    .expect("Expected `MapDefinition` to be loaded.");

                let mut sequence_end_transitions_loader = SequenceEndTransitionsLoader {
                    sequence_end_transition_mapper: sequence_end_transition_mapper_sprite,
                    asset_sequence_end_transitions,
                };

                let background_definition = &map_definition.background;
                if let Some(sprite_sheet_handles) = sprite_sheet_handles {
                    wait_sequence_handles_loader.load(
                        background_definition.layers.values(),
                        |layer| layer.sequence.frames.iter(),
                        asset_id,
                    );
                    sprite_render_sequence_handles_loader.load(
                        background_definition.layers.values(),
                        |layer| layer.sequence.frames.iter(),
                        asset_id,
                        sprite_sheet_handles,
                    );
                    sprite_positions_loader.load(background_definition.layers.values(), asset_id);

                    // Background layers.
                    Self::load_background_layers(
                        asset_sequence_id_mappings_sprite,
                        asset_background_layers,
                        asset_slug,
                        asset_id,
                        background_definition,
                    );
                }

                sequence_end_transitions_loader
                    .load(background_definition.layers.values(), asset_id);

                let margins = Margins::from(map_definition.header.bounds);
                asset_map_bounds.insert(asset_id, map_definition.header.bounds);
                asset_margins.insert(asset_id, margins);
            }
            AssetType::Ui => {
                let background_definition = asset_background_definition_handle
                    .get(asset_id)
                    .and_then(|background_definition_handle| {
                        background_definition_assets.get(background_definition_handle)
                    });

                let ui_definition =
                    asset_ui_definition_handle
                        .get(asset_id)
                        .and_then(|ui_definition_handle| {
                            ui_definition_assets.get(ui_definition_handle)
                        });

                let sprite_positions = {
                    let mut sprite_positions = Vec::new();
                    if let Some(background_definition) = background_definition {
                        sprite_positions.append(
                            &mut <SpritePositionsLoader as SequenceComponentDataLoader>::load(
                                |sequence_ref| *AsRef::<SpritePosition>::as_ref(&sequence_ref),
                                background_definition.layers.values(),
                            ),
                        );
                    }

                    if let Some(ui_definition) = ui_definition {
                        sprite_positions.append(
                            &mut <SpritePositionsLoader as SequenceComponentDataLoader>::load(
                                |sequence_ref| *AsRef::<SpritePosition>::as_ref(&sequence_ref),
                                ui_definition.sequences.values(),
                            ),
                        );
                    }

                    SpritePositions::new(sprite_positions)
                };
                asset_sprite_positions.insert(asset_id, sprite_positions);

                if let Some(ui_definition) = ui_definition {
                    match &ui_definition.ui_type {
                        UiType::Menu(ui_menu_items) => {
                            let ui_menu_items = ui_menu_items
                                .iter()
                                .map(|ui_menu_item| {
                                    let sequence_id_mappings = asset_sequence_id_mappings_ui
                                        .get(asset_id)
                                        .unwrap_or_else(|| {
                                            panic!(
                                                "Expected `SequenceIdMappings<UiSequenceName>` \
                                                 to exist for `{}`.",
                                                asset_slug
                                            )
                                        });
                                    let sequence = &ui_menu_item.sequence;
                                    let sequence_id = sequence_id_mappings
                                        .id(sequence)
                                        .copied()
                                        .unwrap_or_else(|| {
                                            panic!(
                                                "Expected `SequenceIdMapping` to exist for \
                                                 sequence `{}` for asset `{}`.",
                                                sequence, asset_slug
                                            )
                                        });

                                    UiMenuItem::new(
                                        ui_menu_item.index,
                                        ui_menu_item.text.clone(),
                                        sequence_id,
                                    )
                                })
                                .collect::<Vec<UiMenuItem<GameModeIndex>>>();
                            let ui_menu_items = UiMenuItems::new(ui_menu_items);

                            asset_ui_menu_items.insert(asset_id, ui_menu_items);
                        }
                    }
                }

                // Load sequence components from `UiDefinition`.
                let wait_sequence_handles = {
                    let wait_sequence_loader = &wait_sequence_handles_loader.wait_sequence_loader;

                    let mut wait_sequence_handles = Vec::new();

                    if let Some(background_definition) = background_definition {
                        wait_sequence_handles.extend(background_definition.layers.values().map(
                            |layer| {
                                wait_sequence_loader
                                    .load(|frame| frame.wait, layer.sequence.frames.iter())
                            },
                        ));
                    }

                    if let Some(ui_definition) = ui_definition {
                        wait_sequence_handles.extend(ui_definition.sequences.values().map(
                            |sequence| {
                                wait_sequence_loader
                                    .load(|frame| frame.wait, sequence.sequence.frames.iter())
                            },
                        ));
                    };

                    WaitSequenceHandles::new(wait_sequence_handles)
                };
                asset_wait_sequence_handles.insert(asset_id, wait_sequence_handles);

                if let Some(sprite_sheet_handles) = sprite_sheet_handles {
                    let sprite_render_sequence_handles = {
                        let sprite_frame_to_sprite_render = |frame: &SpriteFrame| {
                            let sprite_ref = &frame.sprite;
                            let sprite_sheet = sprite_sheet_handles[sprite_ref.sheet].clone();
                            let sprite_number = sprite_ref.index;
                            SpriteRender {
                                sprite_sheet,
                                sprite_number,
                            }
                        };

                        let sprite_render_sequence_loader =
                            &sprite_render_sequence_handles_loader.sprite_render_sequence_loader;

                        let mut sprite_render_sequence_handles = Vec::new();

                        if let Some(background_definition) = background_definition {
                            sprite_render_sequence_handles.extend(
                                background_definition.layers.values().map(|layer| {
                                    sprite_render_sequence_loader.load(
                                        sprite_frame_to_sprite_render,
                                        layer.sequence.frames.iter(),
                                    )
                                }),
                            );

                            // Background layers.
                            Self::load_background_layers(
                                asset_sequence_id_mappings_ui,
                                asset_background_layers,
                                asset_slug,
                                asset_id,
                                background_definition,
                            );
                        }

                        if let Some(ui_definition) = ui_definition {
                            sprite_render_sequence_handles.extend(
                                ui_definition.sequences.values().map(|sequence| {
                                    sprite_render_sequence_loader.load(
                                        sprite_frame_to_sprite_render,
                                        sequence.sequence.frames.iter(),
                                    )
                                }),
                            );
                        }

                        SpriteRenderSequenceHandles::new(sprite_render_sequence_handles)
                    };

                    asset_sprite_render_sequence_handles
                        .insert(asset_id, sprite_render_sequence_handles);
                }

                let sequence_end_transitions = {
                    let mut sequence_end_transitions = Vec::new();

                    if let Some(background_definition) = background_definition {
                        sequence_end_transitions.extend(background_definition.layers.values().map(
                            |layer| {
                                sequence_end_transition_mapper_ui.map_disparate(asset_id, layer)
                            },
                        ));
                    }

                    if let Some(ui_definition) = ui_definition {
                        sequence_end_transitions.extend(ui_definition.sequences.values().map(
                            |sequence| sequence_end_transition_mapper_ui.map(asset_id, sequence),
                        ));
                    };

                    SequenceEndTransitions::new(sequence_end_transitions)
                };
                asset_sequence_end_transitions.insert(asset_id, sequence_end_transitions);
            }
        }
    }

    /// Returns whether sequence components assets have been loaded.
    fn is_complete(
        _: &AssetLoadingResources<'_>,
        SequenceComponentLoadingResources {
            wait_sequence_assets,
            source_sequence_assets,
            object_acceleration_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
            spawns_sequence_assets,
            asset_wait_sequence_handles,
            asset_source_sequence_handles,
            asset_object_acceleration_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_body_sequence_handles,
            asset_interactions_sequence_handles,
            asset_spawns_sequence_handles,
            ..
        }: &SequenceComponentLoadingResources<'_>,
        asset_id: AssetId,
    ) -> bool {
        macro_rules! sequence_component_loaded {
            ($handleses:ident, $assets:ident) => {{
                if let Some(handles) = $handleses.get(asset_id) {
                    handles.iter().all(|handle| $assets.get(handle).is_some())
                } else {
                    true
                }
            }};
        };

        sequence_component_loaded!(asset_wait_sequence_handles, wait_sequence_assets)
            && sequence_component_loaded!(asset_source_sequence_handles, source_sequence_assets)
            && sequence_component_loaded!(
                asset_object_acceleration_sequence_handles,
                object_acceleration_sequence_assets
            )
            && sequence_component_loaded!(
                asset_sprite_render_sequence_handles,
                sprite_render_sequence_assets
            )
            && sequence_component_loaded!(asset_body_sequence_handles, body_sequence_assets)
            && sequence_component_loaded!(
                asset_interactions_sequence_handles,
                interactions_sequence_assets
            )
            && sequence_component_loaded!(asset_spawns_sequence_handles, spawns_sequence_assets)
    }
}
