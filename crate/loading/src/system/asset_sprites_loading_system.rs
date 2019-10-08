use amethyst::{
    assets::{AssetStorage, Handle, ProgressCounter},
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_loading::YamlFormat;
use asset_model::{config::AssetType, loaded::AssetId};
use derivative::Derivative;
use derive_new::new;
use loading_model::loaded::{AssetLoadStatus, LoadStatus};
use log::debug;
use object_model::config::{GameObjectFrame, GameObjectSequence, ObjectDefinition};
use object_type::ObjectType;
use serde::{Deserialize, Serialize};
use slotmap::SecondaryMap;
use sprite_model::config::SpritesDefinition;
use typename_derive::TypeName;

use crate::{AssetLoadingResources, DefinitionLoadingResources, IdMappingResources};

/// Loads asset sprites definitions.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct AssetSpritesLoadingSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetSpritesLoadingSystemData<'s> {
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_status: Write<'s, AssetLoadStatus>,
    /// `AssetLoadingResources`.
    #[derivative(Debug = "ignore")]
    pub asset_loading_resources: AssetLoadingResources<'s>,
    /// `DefinitionLoadingResources`.
    pub definition_loading_resources: DefinitionLoadingResources<'s>,
    /// `IdMappingResources`.
    pub id_mapping_resources: IdMappingResources<'s>,
    /// `SpriteLoadingResources`.
    pub sprite_loading_resources: SpriteLoadingResources<'s>,
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

impl<'s> System<'s> for AssetSpritesLoadingSystem {
    type SystemData = AssetSpritesLoadingSystemData<'s>;

    fn run(
        &mut self,
        AssetSpritesLoadingSystemData {
            mut asset_load_status,
            mut asset_loading_resources,
            definition_loading_resources,
            id_mapping_resources,
            mut sprite_loading_resources,
        }: Self::SystemData,
    ) {
        asset_load_status
            .iter_mut()
            .filter(|(_, load_status)| **load_status == LoadStatus::IdMapping)
            .for_each(|(asset_id, load_status)| {
                if Self::id_mapped(
                    &asset_loading_resources,
                    &definition_loading_resources,
                    &id_mapping_resources,
                    asset_id,
                ) {
                    Self::sprites_load(
                        &mut asset_loading_resources,
                        &mut sprite_loading_resources,
                        asset_id,
                    );

                    *load_status = LoadStatus::SpritesLoading
                }
            });
    }
}

impl AssetSpritesLoadingSystem {
    /// Returns whether ID mappings has been completed.
    fn id_mapped(
        asset_loading_resources: &AssetLoadingResources<'_>,
        DefinitionLoadingResources {
            character_definition_assets,
            energy_definition_assets,
            asset_character_definition_handle,
            asset_energy_definition_handle,
            ..
        }: &DefinitionLoadingResources<'_>,
        id_mapping_resources: &IdMappingResources<'_>,
        asset_id: AssetId,
    ) -> bool {
        let AssetLoadingResources {
            asset_type_mappings,
            ..
        } = asset_loading_resources;
        let IdMappingResources {
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
        } = id_mapping_resources;

        let asset_type = asset_type_mappings
            .get(asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        match asset_type {
            AssetType::Object(object_type) => match object_type {
                ObjectType::Character => {
                    let id_mappings_self =
                        asset_sequence_id_mappings_character.get(asset_id).is_some();
                    let spawn_id_mappings_exist = {
                        let character_definition = asset_character_definition_handle
                            .get(asset_id)
                            .and_then(|character_definition_handle| {
                                character_definition_assets.get(character_definition_handle)
                            })
                            .expect("Expected `CharacterDefinition` to be loaded.");

                        Self::spawn_object_sequence_id_mappings_loaded(
                            asset_loading_resources,
                            id_mapping_resources,
                            &character_definition.object_definition,
                        )
                    };

                    id_mappings_self && spawn_id_mappings_exist
                }
                ObjectType::Energy => {
                    let id_mappings_self =
                        asset_sequence_id_mappings_energy.get(asset_id).is_some();
                    let spawn_id_mappings_exist = {
                        let energy_definition = asset_energy_definition_handle
                            .get(asset_id)
                            .and_then(|energy_definition_handle| {
                                energy_definition_assets.get(energy_definition_handle)
                            })
                            .expect("Expected `EnergyDefinition` to be loaded.");

                        Self::spawn_object_sequence_id_mappings_loaded(
                            asset_loading_resources,
                            id_mapping_resources,
                            &energy_definition.object_definition,
                        )
                    };

                    id_mappings_self && spawn_id_mappings_exist
                }
                ObjectType::TestObject => panic!("`TestObject` loading is not supported."),
            },
            AssetType::Map => true,
        }
    }

    fn spawn_object_sequence_id_mappings_loaded<ObjSeq>(
        AssetLoadingResources {
            asset_id_mappings,
            asset_type_mappings,
            ..
        }: &AssetLoadingResources<'_>,
        IdMappingResources {
            asset_sequence_id_mappings_character,
            asset_sequence_id_mappings_energy,
        }: &IdMappingResources<'_>,
        object_definition: &ObjectDefinition<ObjSeq>,
    ) -> bool
    where
        ObjSeq: GameObjectSequence,
        ObjSeq::SequenceName: for<'des> Deserialize<'des> + Serialize,
    {
        object_definition
            .sequences
            .values()
            .flat_map(|game_obj_seq| game_obj_seq.object_sequence().frames.iter())
            .flat_map(|frame| frame.object_frame().spawns.iter())
            .try_fold((), |_, spawn| {
                // Check if sequence ID mappings exist for `spawn.object`.
                let spawn_asset_slug = &spawn.object;
                let spawn_asset_id = asset_id_mappings
                    .id(spawn_asset_slug)
                    .copied()
                    .unwrap_or_else(|| panic!("Asset ID not found for `{}`.", spawn_asset_slug));
                let spawn_asset_type = asset_type_mappings
                    .get(spawn_asset_id)
                    .expect("Expected `AssetType` mapping to exist.");

                let spawn_id_mappings_exist = match spawn_asset_type {
                    AssetType::Object(spawn_object_type) => match spawn_object_type {
                        ObjectType::Character => asset_sequence_id_mappings_character
                            .get(spawn_asset_id)
                            .is_some(),
                        ObjectType::Energy => asset_sequence_id_mappings_energy
                            .get(spawn_asset_id)
                            .is_some(),
                        ObjectType::TestObject => {
                            panic!("Spawning `TestObject`s is not supported.")
                        }
                    },
                    AssetType::Map => panic!("Spawning `Map`s is not supported."),
                };
                if spawn_id_mappings_exist {
                    Ok(())
                } else {
                    Err(())
                }
            })
            .is_ok()
    }

    /// Loads an asset's `SpritesDefinition`.
    fn sprites_load(
        AssetLoadingResources {
            asset_id_to_path,
            asset_id_mappings,
            asset_type_mappings,
            load_status_progress_counters,
            loader,
        }: &mut AssetLoadingResources<'_>,
        SpriteLoadingResources {
            sprites_definition_assets,
            asset_sprites_definition_handles,
        }: &mut SpriteLoadingResources<'_>,
        asset_id: AssetId,
    ) {
        let asset_type = asset_type_mappings
            .get(asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        let progress_counter = load_status_progress_counters
            .entry(LoadStatus::SpritesLoading)
            .or_insert_with(ProgressCounter::new);

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
}
