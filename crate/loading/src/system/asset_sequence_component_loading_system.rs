use amethyst::{
    assets::AssetStorage,
    audio::Source,
    ecs::{Read, System, World, Write},
    renderer::SpriteRender,
    shred::{ResourceId, SystemData},
};
use asset_model::{config::AssetType, loaded::AssetId};
use audio_model::loaded::{AssetSourceSequenceHandles, SourceSequence};
use character_loading::{CtsLoader, CtsLoaderParams, CHARACTER_TRANSITIONS_DEFAULT};
use character_model::{
    config::CharacterSequence,
    loaded::{
        AssetCharacterCtsHandles, CharacterControlTransitions, CharacterCts, CharacterCtsHandle,
        CharacterCtsHandles,
    },
};
use collision_model::{
    config::{Body, Interactions},
    loaded::{
        AssetBodySequenceHandles, AssetInteractionsSequenceHandles, BodySequence,
        InteractionsSequence,
    },
};
use derivative::Derivative;
use derive_new::new;
use energy_model::config::EnergySequence;
use kinematic_model::loaded::{AssetObjectAccelerationSequenceHandles, ObjectAccelerationSequence};
use loading_model::loaded::{AssetLoadStage, LoadStage};
use log::debug;
use map_model::{
    config::LayerPosition,
    loaded::{AssetLayerPositions, AssetMapBounds, AssetMargins, LayerPositions, Margins},
};
use object_loading::{ObjectLoader, ObjectLoaderParams};
use object_model::loaded::Object;
use object_type::ObjectType;
use sequence_model::{
    config::Wait,
    loaded::{
        AssetSequenceEndTransitions, AssetWaitSequenceHandles, WaitSequence, WaitSequenceHandle,
        WaitSequenceHandles,
    },
};
use spawn_model::loaded::{AssetSpawnsSequenceHandles, Spawns, SpawnsSequence};
use sprite_model::loaded::{
    AssetSpriteRenderSequenceHandles, SpriteRenderSequence, SpriteRenderSequenceHandle,
    SpriteRenderSequenceHandles,
};
use typename_derive::TypeName;

use crate::{
    AssetLoadingResources, DefinitionLoadingResources, IdMappingResources, TextureLoadingResources,
};

/// Loads asset sequence components.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct AssetSequenceComponentLoadingSystem;

/// `AssetSequenceComponentLoadingSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetSequenceComponentLoadingSystemData<'s> {
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_stage: Write<'s, AssetLoadStage>,
    /// `AssetLoadingResources`.
    #[derivative(Debug = "ignore")]
    pub asset_loading_resources: AssetLoadingResources<'s>,
    /// `DefinitionLoadingResources`.
    pub definition_loading_resources: DefinitionLoadingResources<'s>,
    /// `IdMappingResources`.
    pub id_mapping_resources: IdMappingResources<'s>,
    /// `TextureLoadingResources`.
    pub texture_loading_resources: TextureLoadingResources<'s>,
    /// `SequenceComponentResources`.
    pub sequence_component_resources: SequenceComponentResources<'s>,
}

/// `SequenceComponentResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceComponentResources<'s> {
    /// `Source`s assets.
    #[derivative(Debug = "ignore")]
    pub source_assets: Read<'s, AssetStorage<Source>>,
    /// `Body` assets.
    #[derivative(Debug = "ignore")]
    pub body_assets: Read<'s, AssetStorage<Body>>,
    /// `Interactions` assets.
    #[derivative(Debug = "ignore")]
    pub interactions_assets: Read<'s, AssetStorage<Interactions>>,
    /// `Spawns` assets.
    #[derivative(Debug = "ignore")]
    pub spawns_assets: Read<'s, AssetStorage<Spawns>>,

    /// `WaitSequence` assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: Read<'s, AssetStorage<WaitSequence>>,
    /// `SourceSequence` assets.
    #[derivative(Debug = "ignore")]
    pub source_sequence_assets: Read<'s, AssetStorage<SourceSequence>>,
    /// `ObjectAccelerationSequence` assets.
    #[derivative(Debug = "ignore")]
    pub object_acceleration_sequence_assets: Read<'s, AssetStorage<ObjectAccelerationSequence>>,
    /// `SpriteRenderSequence` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_assets: Read<'s, AssetStorage<SpriteRenderSequence>>,
    /// `BodySequence` assets.
    #[derivative(Debug = "ignore")]
    pub body_sequence_assets: Read<'s, AssetStorage<BodySequence>>,
    /// `InteractionsSequence` assets.
    #[derivative(Debug = "ignore")]
    pub interactions_sequence_assets: Read<'s, AssetStorage<InteractionsSequence>>,
    /// `SpawnsSequence` assets.
    #[derivative(Debug = "ignore")]
    pub spawns_sequence_assets: Read<'s, AssetStorage<SpawnsSequence>>,

    /// `CharacterControlTransitions` assets.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_assets: Read<'s, AssetStorage<CharacterControlTransitions>>,
    /// `CharacterCts` assets.
    #[derivative(Debug = "ignore")]
    pub character_cts_assets: Read<'s, AssetStorage<CharacterCts>>,

    /// `AssetSequenceEndTransitions` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_end_transitions: Write<'s, AssetSequenceEndTransitions>,
    /// `AssetWaitSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_wait_sequence_handles: Write<'s, AssetWaitSequenceHandles>,
    /// `AssetSourceSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_source_sequence_handles: Write<'s, AssetSourceSequenceHandles>,
    /// `AssetObjectAccelerationSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_object_acceleration_sequence_handles:
        Write<'s, AssetObjectAccelerationSequenceHandles>,
    /// `AssetSpriteRenderSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprite_render_sequence_handles: Write<'s, AssetSpriteRenderSequenceHandles>,
    /// `AssetBodySequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_body_sequence_handles: Write<'s, AssetBodySequenceHandles>,
    /// `AssetInteractionsSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_interactions_sequence_handles: Write<'s, AssetInteractionsSequenceHandles>,
    /// `AssetSpawnsSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_spawns_sequence_handles: Write<'s, AssetSpawnsSequenceHandles>,

    /// `AssetCharacterCtsHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_character_cts_handles: Write<'s, AssetCharacterCtsHandles>,

    /// `AssetMapBounds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_bounds: Write<'s, AssetMapBounds>,
    /// `AssetMargins` resource.
    #[derivative(Debug = "ignore")]
    pub asset_margins: Write<'s, AssetMargins>,
    /// `AssetLayerPositions` resource.
    #[derivative(Debug = "ignore")]
    pub asset_layer_positions: Write<'s, AssetLayerPositions>,
}

impl<'s> System<'s> for AssetSequenceComponentLoadingSystem {
    type SystemData = AssetSequenceComponentLoadingSystemData<'s>;

    fn run(
        &mut self,
        AssetSequenceComponentLoadingSystemData {
            mut asset_load_stage,
            mut asset_loading_resources,
            definition_loading_resources,
            id_mapping_resources,
            texture_loading_resources,
            mut sequence_component_resources,
        }: Self::SystemData,
    ) {
        let capacity = asset_loading_resources.asset_id_mappings.capacity();
        let SequenceComponentResources {
            asset_sequence_end_transitions,
            asset_wait_sequence_handles,
            asset_source_sequence_handles,
            asset_object_acceleration_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_body_sequence_handles,
            asset_interactions_sequence_handles,
            asset_spawns_sequence_handles,
            ..
        } = &mut sequence_component_resources;
        asset_sequence_end_transitions.set_capacity(capacity);
        asset_wait_sequence_handles.set_capacity(capacity);
        asset_source_sequence_handles.set_capacity(capacity);
        asset_object_acceleration_sequence_handles.set_capacity(capacity);
        asset_sprite_render_sequence_handles.set_capacity(capacity);
        asset_body_sequence_handles.set_capacity(capacity);
        asset_interactions_sequence_handles.set_capacity(capacity);
        asset_spawns_sequence_handles.set_capacity(capacity);

        asset_load_stage
            .iter_mut()
            .filter(|(_, load_stage)| **load_stage == LoadStage::TextureLoading)
            .for_each(|(asset_id, load_stage)| {
                if Self::textures_loaded(&texture_loading_resources, asset_id) {
                    Self::sequence_components_load(
                        &mut asset_loading_resources,
                        &definition_loading_resources,
                        &id_mapping_resources,
                        &texture_loading_resources,
                        &mut sequence_component_resources,
                        asset_id,
                    );

                    *load_stage = LoadStage::SequenceComponentLoading;
                }
            });
    }
}

impl AssetSequenceComponentLoadingSystem {
    /// Returns whether the `Texture`s and `SpriteSheet` assets have been loaded.
    ///
    /// Returns `true` if there are no textures to load.
    fn textures_loaded(
        TextureLoadingResources {
            texture_assets,
            sprite_sheet_assets,
            asset_sprite_sheet_handles,
        }: &TextureLoadingResources<'_>,
        asset_id: AssetId,
    ) -> bool {
        asset_sprite_sheet_handles
            .get(asset_id)
            .map(|sprite_sheet_handles| {
                sprite_sheet_handles.iter().all(|sprite_sheet_handle| {
                    sprite_sheet_assets
                        .get(sprite_sheet_handle)
                        .and_then(|sprite_sheet| texture_assets.get(&sprite_sheet.texture))
                        .is_some()
                })
            })
            .unwrap_or(true)
    }

    /// Loads an asset's sequence components.
    fn sequence_components_load(
        AssetLoadingResources {
            asset_id_mappings,
            asset_type_mappings,
            loader,
            ..
        }: &mut AssetLoadingResources<'_>,
        DefinitionLoadingResources {
            character_definition_assets,
            energy_definition_assets,
            map_definition_assets,
            asset_character_definition_handle,
            asset_energy_definition_handle,
            asset_map_definition_handle,
        }: &DefinitionLoadingResources<'_>,
        IdMappingResources {
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
        }: &IdMappingResources<'_>,
        TextureLoadingResources {
            asset_sprite_sheet_handles,
            ..
        }: &TextureLoadingResources<'_>,
        SequenceComponentResources {
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
        }: &mut SequenceComponentResources<'_>,
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
                    let capacity = map_definition.layers.len();
                    let sequence_handles = (
                        Vec::<WaitSequenceHandle>::with_capacity(capacity),
                        Vec::<SpriteRenderSequenceHandle>::with_capacity(capacity),
                        Vec::<LayerPosition>::with_capacity(capacity),
                    );
                    let (wait_sequence_handles, sprite_render_sequence_handles, layer_positions) =
                        map_definition.layers.iter().fold(
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
}
