use std::{collections::HashMap, path::PathBuf};

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    ecs::{Read, ReadExpect, System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_loading::YamlFormat;
use asset_model::{
    config::AssetType,
    loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
};
use character_model::config::{CharacterDefinition, CharacterDefinitionHandle};
use derivative::Derivative;
use derive_new::new;
use energy_model::config::{EnergyDefinition, EnergyDefinitionHandle};
use log::debug;
use map_model::config::MapDefinition;
use object_type::ObjectType;
use sequence_model::loaded::{WaitSequence, WaitSequenceHandles};
use slotmap::SecondaryMap;
use typename_derive::TypeName;

use crate::AssetLoadStatus;

/// Loads game object assets.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct AssetLoadingSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetLoadingSystemData<'s> {
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_to_status: Write<'s, SecondaryMap<AssetId, AssetLoadStatus>>,
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
    /// `HashMap<AssetLoadStatus, WaitSequenceHandles>` resource.
    #[derivative(Debug = "ignore")]
    pub load_status_progress_counters: Write<'s, HashMap<AssetLoadStatus, ProgressCounter>>,
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    pub loader: ReadExpect<'s, Loader>,
    /// `DefinitionLoadingResources`.
    pub definition_loading_resources: DefinitionLoadingResources<'s>,
    /// `WaitSequence` assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: Read<'s, AssetStorage<WaitSequence>>,
    /// `SecondaryMap::<AssetId, WaitSequenceHandles>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_wait_sequence_handles: Write<'s, SecondaryMap<AssetId, WaitSequenceHandles>>,
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
    /// `SecondaryMap::<AssetId, CharacterDefinitionHandle>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_character_definition_handles:
        Write<'s, SecondaryMap<AssetId, CharacterDefinitionHandle>>,
    /// `SecondaryMap::<AssetId, EnergyDefinitionHandle>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_energy_definition_handles: Write<'s, SecondaryMap<AssetId, EnergyDefinitionHandle>>,
    /// `SecondaryMap::<AssetId, Handle<MapDefinition>>` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_definition_handles: Write<'s, SecondaryMap<AssetId, Handle<MapDefinition>>>,
}

impl<'s> System<'s> for AssetLoadingSystem {
    type SystemData = AssetLoadingSystemData<'s>;

    fn run(
        &mut self,
        AssetLoadingSystemData {
            mut asset_id_to_status,
            mut asset_loading_resources,
        }: Self::SystemData,
    ) {
        let capacity = asset_loading_resources.asset_id_mappings.capacity();
        asset_loading_resources
            .asset_wait_sequence_handles
            .set_capacity(capacity);

        asset_id_to_status
            .iter_mut()
            .for_each(|(asset_id, asset_load_status)| {
                *asset_load_status = Self::process_asset(
                    &mut asset_loading_resources,
                    &asset_id,
                    *asset_load_status,
                );
            });
    }
}

impl AssetLoadingSystem {
    fn process_asset(
        asset_loading_resources: &mut AssetLoadingResources,
        asset_id: &AssetId,
        asset_load_status: AssetLoadStatus,
    ) -> AssetLoadStatus {
        match asset_load_status {
            AssetLoadStatus::New => {
                Self::definition_load(asset_loading_resources, *asset_id);

                AssetLoadStatus::DefinitionLoading
            }
            AssetLoadStatus::DefinitionLoading => unimplemented!(),
            AssetLoadStatus::SpritesLoading => unimplemented!(),
            AssetLoadStatus::SequenceComponentLoading => unimplemented!(),
            AssetLoadStatus::Complete => AssetLoadStatus::Complete,
        }
    }

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
                    ref mut asset_character_definition_handles,
                    ref mut asset_energy_definition_handles,
                    ref mut asset_map_definition_handles,
                },
            ..
        }: &mut AssetLoadingResources,
        asset_id: AssetId,
    ) {
        let asset_type = asset_type_mappings
            .asset_type(&asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        let progress_counter = load_status_progress_counters
            .entry(AssetLoadStatus::DefinitionLoading)
            .or_insert(ProgressCounter::new());

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");
        let asset_path = asset_id_to_path
            .get(asset_id)
            .expect("Expected `PathBuf` mapping to exist for `AssetId`.");
        debug!("Loading `{}` from: `{}`", asset_slug, asset_path.display());

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

                        asset_character_definition_handles
                            .insert(asset_id, character_definition_handle);
                    }
                    ObjectType::Energy => {
                        let energy_definition_handle = loader.load(
                            object_definition_path,
                            YamlFormat,
                            &mut *progress_counter,
                            energy_definition_assets,
                        );

                        asset_energy_definition_handles.insert(asset_id, energy_definition_handle);
                    }
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

                asset_map_definition_handles.insert(asset_id, map_definition_handle);
            }
        }
    }
}
