use amethyst::{
    assets::{AssetStorage, ProgressCounter},
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_loading::YamlFormat;
use asset_model::{config::AssetType, loaded::AssetId};
use character_model::{config::CharacterDefinition, loaded::AssetCharacterDefinitionHandle};
use derivative::Derivative;
use derive_new::new;
use energy_model::{config::EnergyDefinition, loaded::AssetEnergyDefinitionHandle};
use loading_model::loaded::{AssetLoadStage, LoadStage};
use log::debug;
use map_model::{config::MapDefinition, loaded::AssetMapDefinitionHandle};
use object_type::ObjectType;
use typename_derive::TypeName;

use crate::AssetLoadingResources;

/// Loads asset definitions.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct AssetDefinitionLoadingSystem;

/// `AssetDefinitionLoadingSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetDefinitionLoadingSystemData<'s> {
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_stage: Write<'s, AssetLoadStage>,
    /// `AssetLoadingResources`.
    pub asset_loading_resources: AssetLoadingResources<'s>,
    /// `DefinitionLoadingResources`.
    pub definition_loading_resources: DefinitionLoadingResources<'s>,
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

impl<'s> System<'s> for AssetDefinitionLoadingSystem {
    type SystemData = AssetDefinitionLoadingSystemData<'s>;

    fn run(
        &mut self,
        AssetDefinitionLoadingSystemData {
            mut asset_load_stage,
            mut asset_loading_resources,
            mut definition_loading_resources,
        }: Self::SystemData,
    ) {
        asset_load_stage
            .iter_mut()
            .filter(|(_, load_stage)| **load_stage == LoadStage::New)
            .for_each(|(asset_id, load_stage)| {
                Self::definition_load(
                    &mut asset_loading_resources,
                    &mut definition_loading_resources,
                    asset_id,
                );

                *load_stage = LoadStage::DefinitionLoading;
            });
    }
}

impl AssetDefinitionLoadingSystem {
    /// Loads an asset's `Definition`.
    fn definition_load(
        AssetLoadingResources {
            asset_id_to_path,
            asset_id_mappings,
            asset_type_mappings,
            load_stage_progress_counters,
            loader,
        }: &mut AssetLoadingResources<'_>,
        DefinitionLoadingResources {
            character_definition_assets,
            energy_definition_assets,
            map_definition_assets,
            asset_character_definition_handle,
            asset_energy_definition_handle,
            asset_map_definition_handle,
        }: &mut DefinitionLoadingResources<'_>,
        asset_id: AssetId,
    ) {
        debug!("Loading asset with ID: {:?}", asset_id);

        let asset_type = asset_type_mappings
            .get(asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        let progress_counter = load_stage_progress_counters
            .entry(LoadStage::DefinitionLoading)
            .or_insert_with(ProgressCounter::new);

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
}
