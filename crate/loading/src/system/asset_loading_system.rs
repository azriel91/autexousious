use std::{collections::HashMap, iter::FromIterator, path::PathBuf};

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    audio::Source,
    ecs::{Read, ReadExpect, System, World, Write},
    renderer::{sprite::SpriteSheetHandle, SpriteRender, SpriteSheet, Texture},
    shred::{ResourceId, SystemData},
};
use asset_loading::YamlFormat;
use asset_model::{
    config::AssetType,
    loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
};
use audio_model::loaded::{AssetSourceSequenceHandles, SourceSequence};
use character_loading::{CtsLoader, CtsLoaderParams, CHARACTER_TRANSITIONS_DEFAULT};
use character_model::{
    config::{CharacterDefinition, CharacterSequence, CharacterSequenceName},
    loaded::{
        AssetCharacterCtsHandles, AssetCharacterDefinitionHandle, CharacterControlTransitions,
        CharacterCts, CharacterCtsHandle, CharacterCtsHandles,
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
use energy_model::{
    config::{EnergyDefinition, EnergySequence, EnergySequenceName},
    loaded::AssetEnergyDefinitionHandle,
};
use kinematic_model::loaded::{AssetObjectAccelerationSequenceHandles, ObjectAccelerationSequence};
use loading_model::loaded::{AssetLoadStatus, LoadStatus};
use log::{debug, info};
use map_model::{
    config::{LayerPosition, MapDefinition},
    loaded::{
        AssetLayerPositions, AssetMapBounds, AssetMapDefinitionHandle, AssetMargins,
        LayerPositions, Margins,
    },
};
use object_loading::{ObjectLoader, ObjectLoaderParams};
use object_model::loaded::Object;
use object_type::ObjectType;
use sequence_model::{
    config::Wait,
    loaded::{
        AssetSequenceEndTransitions, AssetSequenceIdMappings, AssetWaitSequenceHandles,
        SequenceIdMappings, WaitSequence, WaitSequenceHandle, WaitSequenceHandles,
    },
};
use slotmap::SecondaryMap;
use spawn_model::{
    config::Spawns,
    loaded::{AssetSpawnsSequenceHandles, SpawnsSequence},
};
use sprite_loading::SpriteLoader;
use sprite_model::{
    config::SpritesDefinition,
    loaded::{
        AssetSpriteRenderSequenceHandles, SpriteRenderSequence, SpriteRenderSequenceHandle,
        SpriteRenderSequenceHandles,
    },
};
use typename_derive::TypeName;

/// Loads game object assets.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct AssetLoadingSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetLoadingSystemData<'s> {
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_status: Write<'s, AssetLoadStatus>,
    /// `AssetLoadingResources`.
    #[derivative(Debug = "ignore")]
    pub asset_loading_resources: AssetLoadingResources<'s>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetLoadingResources<'s> {
    /// `SecondaryMap<AssetId, PathBuf>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_to_path: Write<'s, SecondaryMap<AssetId, PathBuf>>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
    /// `HashMap<LoadStatus, WaitSequenceHandles>` resource.
    #[derivative(Debug = "ignore")]
    pub load_status_progress_counters: Write<'s, HashMap<LoadStatus, ProgressCounter>>,
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: ReadExpect<'s, Loader>,
    /// `DefinitionLoadingResources`.
    pub definition_loading_resources: DefinitionLoadingResources<'s>,
    /// `SpriteLoadingResources`.
    pub sprite_loading_resources: SpriteLoadingResources<'s>,
    /// `TextureLoadingResources`.
    pub texture_loading_resources: TextureLoadingResources<'s>,
    /// `SequenceComponentResources`.
    pub sequence_component_resources: SequenceComponentResources<'s>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct DefinitionLoadingResources<'s> {
    /// `CharacterDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub character_definition_assets: Read<'s, AssetStorage<CharacterDefinition>>,
    /// `EnergyDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub energy_definition_assets: Read<'s, AssetStorage<EnergyDefinition>>,
    /// `MapDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub map_definition_assets: Read<'s, AssetStorage<MapDefinition>>,
    /// `AssetCharacterDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_character_definition_handle: Write<'s, AssetCharacterDefinitionHandle>,
    /// `AssetEnergyDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_energy_definition_handle: Write<'s, AssetEnergyDefinitionHandle>,
    /// `AssetMapDefinitionHandle` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_definition_handle: Write<'s, AssetMapDefinitionHandle>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpriteLoadingResources<'s> {
    /// `SpritesDefinition` assets.
    #[derivative(Debug = "ignore")]
    pub sprites_definition_assets: Read<'s, AssetStorage<SpritesDefinition>>,
    /// `SecondaryMap<AssetId, Handle<SpritesDefinition>>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprites_definition_handles:
        Write<'s, SecondaryMap<AssetId, Handle<SpritesDefinition>>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct TextureLoadingResources<'s> {
    /// `Texture` assets.
    #[derivative(Debug = "ignore")]
    pub texture_assets: Read<'s, AssetStorage<Texture>>,
    /// `SpriteSheet` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_sheet_assets: Read<'s, AssetStorage<SpriteSheet>>,
    /// `SecondaryMap<AssetId, Vec<SpriteSheetHandle>>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprite_sheet_handles: Write<'s, SecondaryMap<AssetId, Vec<SpriteSheetHandle>>>,
}

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

    /// `AssetSequenceIdMappings<CharacterSequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_character:
        Write<'s, AssetSequenceIdMappings<CharacterSequenceName>>,
    /// `AssetSequenceIdMappings<EnergySequenceName>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_id_mappings_energy: Write<'s, AssetSequenceIdMappings<EnergySequenceName>>,
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

impl<'s> System<'s> for AssetLoadingSystem {
    type SystemData = AssetLoadingSystemData<'s>;

    fn run(
        &mut self,
        AssetLoadingSystemData {
            mut asset_load_status,
            mut asset_loading_resources,
        }: Self::SystemData,
    ) {
        let capacity = asset_loading_resources.asset_id_mappings.capacity();
        let SequenceComponentResources {
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
            asset_sequence_end_transitions,
            asset_wait_sequence_handles,
            asset_source_sequence_handles,
            asset_object_acceleration_sequence_handles,
            asset_sprite_render_sequence_handles,
            asset_body_sequence_handles,
            asset_interactions_sequence_handles,
            asset_spawns_sequence_handles,
            ..
        } = &mut asset_loading_resources.sequence_component_resources;
        asset_sequence_id_mappings_character.set_capacity(capacity);
        asset_sequence_id_mappings_energy.set_capacity(capacity);
        asset_sequence_end_transitions.set_capacity(capacity);
        asset_wait_sequence_handles.set_capacity(capacity);
        asset_source_sequence_handles.set_capacity(capacity);
        asset_object_acceleration_sequence_handles.set_capacity(capacity);
        asset_sprite_render_sequence_handles.set_capacity(capacity);
        asset_body_sequence_handles.set_capacity(capacity);
        asset_interactions_sequence_handles.set_capacity(capacity);
        asset_spawns_sequence_handles.set_capacity(capacity);

        asset_load_status
            .iter_mut()
            .for_each(|(asset_id, load_status)| {
                *load_status =
                    Self::process_asset(&mut asset_loading_resources, &asset_id, *load_status);
            });
    }
}

impl AssetLoadingSystem {
    fn process_asset(
        asset_loading_resources: &mut AssetLoadingResources,
        asset_id: &AssetId,
        load_status: LoadStatus,
    ) -> LoadStatus {
        let asset_id = *asset_id;
        match load_status {
            LoadStatus::New => {
                Self::definition_load(asset_loading_resources, asset_id);

                LoadStatus::DefinitionLoading
            }
            LoadStatus::DefinitionLoading => {
                if Self::definition_loaded(asset_loading_resources, asset_id) {
                    Self::sprites_load(asset_loading_resources, asset_id);

                    LoadStatus::SpritesLoading
                } else {
                    LoadStatus::DefinitionLoading
                }
            }
            LoadStatus::SpritesLoading => {
                if Self::sprites_definition_loaded(asset_loading_resources, asset_id) {
                    Self::texture_load(asset_loading_resources, asset_id);

                    LoadStatus::TextureLoading
                } else {
                    LoadStatus::SpritesLoading
                }
            }
            LoadStatus::TextureLoading => {
                if Self::textures_loaded(asset_loading_resources, asset_id) {
                    Self::sequence_components_load(asset_loading_resources, asset_id);

                    LoadStatus::SequenceComponentLoading
                } else {
                    LoadStatus::TextureLoading
                }
            }
            LoadStatus::SequenceComponentLoading => {
                if Self::sequence_components_loaded(asset_loading_resources, asset_id) {
                    let asset_slug = asset_loading_resources
                        .asset_id_mappings
                        .slug(asset_id)
                        .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");

                    info!("Loaded `{}`.", asset_slug);

                    LoadStatus::Complete
                } else {
                    LoadStatus::SequenceComponentLoading
                }
            }
            LoadStatus::Complete => LoadStatus::Complete,
        }
    }

    /// Loads an asset's `Definition`.
    fn definition_load(
        AssetLoadingResources {
            ref asset_id_to_path,
            ref asset_id_mappings,
            ref asset_type_mappings,
            ref mut load_status_progress_counters,
            ref loader,
            definition_loading_resources:
                DefinitionLoadingResources {
                    ref character_definition_assets,
                    ref energy_definition_assets,
                    ref map_definition_assets,
                    ref mut asset_character_definition_handle,
                    ref mut asset_energy_definition_handle,
                    ref mut asset_map_definition_handle,
                },
            ..
        }: &mut AssetLoadingResources,
        asset_id: AssetId,
    ) {
        debug!("Loading asset with ID: {:?}", asset_id);

        let asset_type = asset_type_mappings
            .get(&asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        let progress_counter = load_status_progress_counters
            .entry(LoadStatus::DefinitionLoading)
            .or_insert(ProgressCounter::new());

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");
        let asset_path = asset_id_to_path
            .get(asset_id)
            .expect("Expected `PathBuf` mapping to exist for `AssetId`.");
        debug!(
            "Loading `{}` definition from: `{}`",
            asset_slug,
            asset_path.display()
        );

        match asset_type {
            AssetType::Object(object_type) => {
                let object_definition_path = asset_path.join("object.yaml");
                let object_definition_path = object_definition_path
                    .to_str()
                    .expect("Expected path to be valid unicode.");

                match object_type {
                    ObjectType::Character => {
                        let character_definition_handle = loader.load(
                            object_definition_path,
                            YamlFormat,
                            &mut *progress_counter,
                            character_definition_assets,
                        );

                        asset_character_definition_handle
                            .insert(asset_id, character_definition_handle);
                    }
                    ObjectType::Energy => {
                        let energy_definition_handle = loader.load(
                            object_definition_path,
                            YamlFormat,
                            &mut *progress_counter,
                            energy_definition_assets,
                        );

                        asset_energy_definition_handle.insert(asset_id, energy_definition_handle);
                    }
                    ObjectType::TestObject => panic!("`TestObject` loading is not supported."),
                }
            }
            AssetType::Map => {
                let map_definition_handle = loader.load(
                    asset_path
                        .join("map.yaml")
                        .to_str()
                        .expect("Expected path to be valid unicode."),
                    YamlFormat,
                    &mut *progress_counter,
                    map_definition_assets,
                );

                asset_map_definition_handle.insert(asset_id, map_definition_handle);
            }
        }
    }

    /// Returns whether the definition asset has been loaded.
    fn definition_loaded(
        AssetLoadingResources {
            ref asset_type_mappings,
            definition_loading_resources:
                DefinitionLoadingResources {
                    ref character_definition_assets,
                    ref energy_definition_assets,
                    ref map_definition_assets,
                    ref mut asset_character_definition_handle,
                    ref mut asset_energy_definition_handle,
                    ref mut asset_map_definition_handle,
                },
            ..
        }: &mut AssetLoadingResources,
        asset_id: AssetId,
    ) -> bool {
        let asset_type = asset_type_mappings
            .get(&asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        match asset_type {
            AssetType::Object(object_type) => match object_type {
                ObjectType::Character => {
                    let character_definition_handle = asset_character_definition_handle
                        .get(asset_id)
                        .expect("Expected `CharacterDefinitionHandle` to exist.");
                    character_definition_assets
                        .get(character_definition_handle)
                        .is_some()
                }
                ObjectType::Energy => {
                    let energy_definition_handle = asset_energy_definition_handle
                        .get(asset_id)
                        .expect("Expected `EnergyDefinitionHandle` to exist.");
                    energy_definition_assets
                        .get(energy_definition_handle)
                        .is_some()
                }
                ObjectType::TestObject => panic!("`TestObject` loading is not supported."),
            },
            AssetType::Map => {
                let map_definition_handle = asset_map_definition_handle
                    .get(asset_id)
                    .expect("Expected `MapDefinitionHandle` to exist.");
                map_definition_assets.get(map_definition_handle).is_some()
            }
        }
    }

    /// Loads an asset's `SpritesDefinition`.
    fn sprites_load(
        AssetLoadingResources {
            ref asset_id_to_path,
            ref asset_id_mappings,
            ref asset_type_mappings,
            ref mut load_status_progress_counters,
            ref loader,
            sprite_loading_resources:
                SpriteLoadingResources {
                    ref sprites_definition_assets,
                    ref mut asset_sprites_definition_handles,
                },
            ..
        }: &mut AssetLoadingResources,
        asset_id: AssetId,
    ) {
        let asset_type = asset_type_mappings
            .get(&asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        let progress_counter = load_status_progress_counters
            .entry(LoadStatus::SpritesLoading)
            .or_insert(ProgressCounter::new());

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");
        let asset_path = asset_id_to_path
            .get(asset_id)
            .expect("Expected `PathBuf` mapping to exist for `AssetId`.");

        let sprites_definition_path = asset_path.join("sprites.yaml");
        if let AssetType::Map = asset_type {
            // Return early if `sprites.yaml` does not exist.
            // This means `asset_sprites_definition_handles` will not have a key for the current
            // `asset_id`.
            if !sprites_definition_path.exists() {
                return;
            }
        }

        let sprites_definition_path = sprites_definition_path
            .to_str()
            .expect("Expected path to be valid unicode.");

        debug!(
            "Loading `{}` sprites definition from: `{}`",
            asset_slug,
            asset_path.display()
        );

        let sprites_definition_handle = loader.load(
            sprites_definition_path,
            YamlFormat,
            &mut *progress_counter,
            sprites_definition_assets,
        );

        asset_sprites_definition_handles.insert(asset_id, sprites_definition_handle);
    }

    /// Returns whether the `SpritesDefinition` asset has been loaded.
    ///
    /// Returns `true` if there was no sprite definition for the asset.
    fn sprites_definition_loaded(
        AssetLoadingResources {
            sprite_loading_resources:
                SpriteLoadingResources {
                    ref sprites_definition_assets,
                    ref mut asset_sprites_definition_handles,
                },
            ..
        }: &mut AssetLoadingResources,
        asset_id: AssetId,
    ) -> bool {
        asset_sprites_definition_handles
            .get(asset_id)
            .map(|sprites_definition_handle| {
                sprites_definition_assets
                    .get(sprites_definition_handle)
                    .is_some()
            })
            .unwrap_or(true)
    }

    /// Loads an asset's `Texture`s and `SpriteSheet`s.
    fn texture_load(
        AssetLoadingResources {
            ref asset_id_to_path,
            ref asset_id_mappings,
            ref mut load_status_progress_counters,
            ref loader,
            sprite_loading_resources:
                SpriteLoadingResources {
                    ref sprites_definition_assets,
                    ref asset_sprites_definition_handles,
                },
            texture_loading_resources:
                TextureLoadingResources {
                    ref texture_assets,
                    ref sprite_sheet_assets,
                    ref mut asset_sprite_sheet_handles,
                },
            ..
        }: &mut AssetLoadingResources,
        asset_id: AssetId,
    ) {
        let mut progress_counter = load_status_progress_counters
            .entry(LoadStatus::TextureLoading)
            .or_insert(ProgressCounter::new());

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");
        let asset_path = asset_id_to_path
            .get(asset_id)
            .expect("Expected `PathBuf` mapping to exist for `AssetId`.");

        let sprites_definition =
            asset_sprites_definition_handles
                .get(asset_id)
                .and_then(|sprites_definition_handle| {
                    sprites_definition_assets.get(sprites_definition_handle)
                });

        if let Some(sprites_definition) = sprites_definition {
            debug!(
                "Loading `{}` textures from: `{}`",
                asset_slug,
                asset_path.display()
            );

            let sprite_sheet_handles = SpriteLoader::load(
                &mut progress_counter,
                &loader,
                &texture_assets,
                &sprite_sheet_assets,
                &sprites_definition,
                &asset_path,
            )
            .expect("Failed to load textures and sprite sheets.");

            asset_sprite_sheet_handles.insert(asset_id, sprite_sheet_handles);
        }
    }

    /// Returns whether the `Texture`s and `SpriteSheet` assets have been loaded.
    ///
    /// Returns `true` if there are no textures to load.
    fn textures_loaded(
        AssetLoadingResources {
            texture_loading_resources:
                TextureLoadingResources {
                    ref texture_assets,
                    ref sprite_sheet_assets,
                    ref mut asset_sprite_sheet_handles,
                },
            ..
        }: &mut AssetLoadingResources,
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
            ref asset_id_mappings,
            ref asset_type_mappings,
            ref loader,
            definition_loading_resources:
                DefinitionLoadingResources {
                    ref character_definition_assets,
                    ref energy_definition_assets,
                    ref map_definition_assets,
                    ref mut asset_character_definition_handle,
                    ref mut asset_energy_definition_handle,
                    ref mut asset_map_definition_handle,
                },
            texture_loading_resources:
                TextureLoadingResources {
                    ref mut asset_sprite_sheet_handles,
                    ..
                },
            sequence_component_resources:
                SequenceComponentResources {
                    ref source_assets,
                    ref body_assets,
                    ref interactions_assets,
                    ref spawns_assets,
                    ref wait_sequence_assets,
                    ref source_sequence_assets,
                    ref object_acceleration_sequence_assets,
                    ref sprite_render_sequence_assets,
                    ref body_sequence_assets,
                    ref interactions_sequence_assets,
                    ref spawns_sequence_assets,
                    ref character_control_transitions_assets,
                    ref character_cts_assets,
                    ref mut asset_sequence_id_mappings_character,
                    ref mut asset_sequence_id_mappings_energy,
                    ref mut asset_sequence_end_transitions,
                    ref mut asset_wait_sequence_handles,
                    ref mut asset_source_sequence_handles,
                    ref mut asset_object_acceleration_sequence_handles,
                    ref mut asset_sprite_render_sequence_handles,
                    ref mut asset_body_sequence_handles,
                    ref mut asset_interactions_sequence_handles,
                    ref mut asset_spawns_sequence_handles,
                    ref mut asset_character_cts_handles,
                    ref mut asset_map_bounds,
                    ref mut asset_margins,
                    ref mut asset_layer_positions,
                },
            ..
        }: &mut AssetLoadingResources,
        asset_id: AssetId,
    ) {
        let asset_type = asset_type_mappings
            .get(&asset_id)
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

                        let sequence_id_mappings = SequenceIdMappings::from_iter(
                            character_definition.object_definition.sequences.keys(),
                        );

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
                                    let cts_handle = CtsLoader::load(
                                        &cts_loader_params,
                                        &sequence_id_mappings,
                                        sequence_default,
                                        sequence,
                                    );
                                    cts_handle
                                })
                                .collect::<Vec<CharacterCtsHandle>>();
                            CharacterCtsHandles::new(character_cts_handles)
                        };
                        asset_character_cts_handles.insert(asset_id, character_cts_handles);

                        asset_sequence_id_mappings_character.insert(asset_id, sequence_id_mappings);

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

                        let sequence_id_mappings = SequenceIdMappings::from_iter(
                            energy_definition.object_definition.sequences.keys(),
                        );
                        asset_sequence_id_mappings_energy.insert(asset_id, sequence_id_mappings);

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

    /// Returns whether sequence components assets have been loaded.
    fn sequence_components_loaded(
        AssetLoadingResources {
            sequence_component_resources:
                SequenceComponentResources {
                    ref wait_sequence_assets,
                    ref source_sequence_assets,
                    ref object_acceleration_sequence_assets,
                    ref sprite_render_sequence_assets,
                    ref body_sequence_assets,
                    ref interactions_sequence_assets,
                    ref spawns_sequence_assets,
                    ref asset_wait_sequence_handles,
                    ref asset_source_sequence_handles,
                    ref asset_object_acceleration_sequence_handles,
                    ref asset_sprite_render_sequence_handles,
                    ref asset_body_sequence_handles,
                    ref asset_interactions_sequence_handles,
                    ref asset_spawns_sequence_handles,
                    ..
                },
            ..
        }: &mut AssetLoadingResources,
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
