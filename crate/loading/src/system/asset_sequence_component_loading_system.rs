use amethyst::renderer::SpriteRender;
use asset_model::{config::AssetType, loaded::AssetId};
use background_model::{config::LayerPosition, loaded::LayerPositions};
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
use sequence_model::{
    config::Wait,
    loaded::{WaitSequence, WaitSequenceHandle, WaitSequenceHandles},
};

use sprite_model::loaded::{
    SpriteRenderSequence, SpriteRenderSequenceHandle, SpriteRenderSequenceHandles,
};
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
        AssetLoadingResources {
            asset_id_mappings,
            asset_type_mappings,
            loader,
            ..
        }: &mut AssetLoadingResources<'_>,
        SequenceComponentLoadingResources {
            definition_loading_resources_read:
                DefinitionLoadingResourcesRead {
                    character_definition_assets,
                    energy_definition_assets,
                    map_definition_assets,
                    asset_character_definition_handle,
                    asset_energy_definition_handle,
                    asset_map_definition_handle,
                },
            id_mapping_resources_read:
                IdMappingResourcesRead {
                    asset_sequence_id_mappings_character,
                    asset_sequence_id_mappings_energy,
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
        let asset_type = asset_type_mappings
            .get(asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");

        debug!("Loading `{}` sequence components.", asset_slug,);

        let sprite_sheet_handles = asset_sprite_sheet_handles.get(asset_id);
        match asset_type {
            AssetType::Object(object_type) => {
                let sprite_sheet_handles = sprite_sheet_handles
                    .expect("Expected `SpriteSheetHandles` to exist for object.");
                let object_loader_params = ObjectLoaderParams {
                    loader: &*loader,
                    asset_id_mappings: &*asset_id_mappings,
                    asset_type_mappings: &*asset_type_mappings,
                    asset_sequence_id_mappings_character: &*asset_sequence_id_mappings_character,
                    asset_sequence_id_mappings_energy: &*asset_sequence_id_mappings_energy,
                    wait_sequence_assets: &*wait_sequence_assets,
                    source_assets: &*source_assets,
                    source_sequence_assets: &*source_sequence_assets,
                    object_acceleration_sequence_assets: &*object_acceleration_sequence_assets,
                    sprite_render_sequence_assets: &*sprite_render_sequence_assets,
                    body_sequence_assets: &*body_sequence_assets,
                    interactions_sequence_assets: &*interactions_sequence_assets,
                    spawns_sequence_assets: &*spawns_sequence_assets,
                    body_assets: &*body_assets,
                    interactions_assets: &*interactions_assets,
                    spawns_assets: &*spawns_assets,
                    sprite_sheet_handles: &sprite_sheet_handles,
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

                if let Some(sprite_sheet_handles) = asset_sprite_sheet_handles.get(asset_id) {
                    let capacity = map_definition.background.layers.len();
                    let sequence_handles = (
                        Vec::<WaitSequenceHandle>::with_capacity(capacity),
                        Vec::<SpriteRenderSequenceHandle>::with_capacity(capacity),
                        Vec::<LayerPosition>::with_capacity(capacity),
                    );
                    let (wait_sequence_handles, sprite_render_sequence_handles, layer_positions) =
                        map_definition.background.layers.iter().fold(
                            sequence_handles,
                            |(
                                mut wait_sequence_handles,
                                mut sprite_render_sequence_handles,
                                mut layer_positions,
                            ),
                             layer| {
                                let wait_sequence = WaitSequence::new(
                                    layer
                                        .frames
                                        .iter()
                                        .map(|frame| frame.wait)
                                        .collect::<Vec<Wait>>(),
                                );
                                let sprite_render_sequence = SpriteRenderSequence::new(
                                    layer
                                        .frames
                                        .iter()
                                        .map(|frame| {
                                            let sprite_ref = &frame.sprite;
                                            let sprite_sheet =
                                                sprite_sheet_handles[sprite_ref.sheet].clone();
                                            let sprite_number = sprite_ref.index;
                                            SpriteRender {
                                                sprite_sheet,
                                                sprite_number,
                                            }
                                        })
                                        .collect::<Vec<SpriteRender>>(),
                                );

                                let wait_sequence_handle =
                                    loader.load_from_data(wait_sequence, (), &wait_sequence_assets);
                                let sprite_render_sequence_handle = loader.load_from_data(
                                    sprite_render_sequence,
                                    (),
                                    &sprite_render_sequence_assets,
                                );

                                wait_sequence_handles.push(wait_sequence_handle);
                                sprite_render_sequence_handles.push(sprite_render_sequence_handle);
                                layer_positions.push(layer.position);

                                (
                                    wait_sequence_handles,
                                    sprite_render_sequence_handles,
                                    layer_positions,
                                )
                            },
                        );
                    let wait_sequence_handles = WaitSequenceHandles::new(wait_sequence_handles);
                    let sprite_render_sequence_handles =
                        SpriteRenderSequenceHandles::new(sprite_render_sequence_handles);
                    let layer_positions = LayerPositions::new(layer_positions);

                    asset_wait_sequence_handles.insert(asset_id, wait_sequence_handles);
                    asset_sprite_render_sequence_handles
                        .insert(asset_id, sprite_render_sequence_handles);
                    asset_layer_positions.insert(asset_id, layer_positions);
                }

                let margins = Margins::from(map_definition.header.bounds);
                asset_map_bounds.insert(asset_id, map_definition.header.bounds);
                asset_margins.insert(asset_id, margins);
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
