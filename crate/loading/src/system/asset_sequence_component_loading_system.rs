use amethyst::renderer::SpriteRender;
use asset_model::{config::AssetType, loaded::AssetId};
use background_model::{
    config::{BackgroundDefinition, LayerPosition},
    loaded::{AssetLayerPositions, LayerPositions},
};
use character_loading::{CtsLoader, CtsLoaderParams, CHARACTER_TRANSITIONS_DEFAULT};
use character_model::{
    config::CharacterSequence,
    loaded::{CharacterCtsHandle, CharacterCtsHandles},
};
use energy_model::config::EnergySequence;
use loading_model::loaded::LoadStage;
use log::debug;
use map_model::loaded::Margins;
use object_loading::{ObjectLoader, ObjectLoaderParams};
use object_model::loaded::Object;
use object_type::ObjectType;
use sequence_loading::{WaitSequenceHandlesLoader, WaitSequenceLoader};
use sequence_model::loaded::WaitSequenceHandles;
use sprite_loading::{SpriteRenderSequenceHandlesLoader, SpriteRenderSequenceLoader};
use sprite_model::{config::SpriteFrame, loaded::SpriteRenderSequenceHandles};
use typename_derive::TypeName;

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
    fn load_layer_positions(
        asset_layer_positions: &mut AssetLayerPositions,
        background_definition: &BackgroundDefinition,
        asset_id: AssetId,
    ) {
        let layer_positions = {
            let capacity = background_definition.layers.len();
            let sequence_handles = Vec::<LayerPosition>::with_capacity(capacity);
            let layer_positions = background_definition.layers.iter().fold(
                sequence_handles,
                |mut layer_positions, layer| {
                    layer_positions.push(layer.position);
                    layer_positions
                },
            );
            LayerPositions::new(layer_positions)
        };

        asset_layer_positions.insert(asset_id, layer_positions);
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
                    asset_sequence_id_mappings_character,
                    asset_sequence_id_mappings_energy,
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
            asset_map_bounds,
            asset_margins,
            asset_layer_positions,
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

                let background_definition = &map_definition.background;
                if let Some(sprite_sheet_handles) = sprite_sheet_handles {
                    wait_sequence_handles_loader.load(
                        background_definition.layers.iter(),
                        |layer| layer.frames.iter(),
                        asset_id,
                    );
                    sprite_render_sequence_handles_loader.load(
                        background_definition.layers.iter(),
                        |layer| layer.frames.iter(),
                        asset_id,
                        sprite_sheet_handles,
                    );

                    Self::load_layer_positions(
                        asset_layer_positions,
                        background_definition,
                        asset_id,
                    );
                }

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

                if let Some(background_definition) = background_definition {
                    Self::load_layer_positions(
                        asset_layer_positions,
                        background_definition,
                        asset_id,
                    );
                }

                // Load menu items.
                let ui_definition =
                    asset_ui_definition_handle
                        .get(asset_id)
                        .and_then(|ui_definition_handle| {
                            ui_definition_assets.get(ui_definition_handle)
                        });

                // Load sequence components from `UiDefinition`.
                let wait_sequence_handles = {
                    let wait_sequence_loader = &wait_sequence_handles_loader.wait_sequence_loader;

                    let mut wait_sequence_handles = Vec::new();

                    if let Some(background_definition) = background_definition {
                        wait_sequence_handles.extend(background_definition.layers.iter().map(
                            |layer| {
                                wait_sequence_loader.load(|frame| frame.wait, layer.frames.iter())
                            },
                        ));
                    }

                    if let Some(ui_definition) = ui_definition {
                        wait_sequence_handles.extend(ui_definition.sequences.values().map(
                            |sequence| {
                                wait_sequence_loader
                                    .load(|frame| frame.wait, sequence.frames.iter())
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
                                background_definition.layers.iter().map(|layer| {
                                    sprite_render_sequence_loader
                                        .load(sprite_frame_to_sprite_render, layer.frames.iter())
                                }),
                            );
                        }

                        if let Some(ui_definition) = ui_definition {
                            sprite_render_sequence_handles.extend(
                                ui_definition.sequences.values().map(|sequence| {
                                    sprite_render_sequence_loader
                                        .load(sprite_frame_to_sprite_render, sequence.frames.iter())
                                }),
                            );
                        }

                        SpriteRenderSequenceHandles::new(sprite_render_sequence_handles)
                    };

                    asset_sprite_render_sequence_handles
                        .insert(asset_id, sprite_render_sequence_handles);
                }
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
