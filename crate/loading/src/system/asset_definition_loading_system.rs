use amethyst::assets::ProgressCounter;
use asset_loading::YamlFormat;
use asset_model::{config::AssetType, loaded::AssetId};
use loading_model::loaded::LoadStage;
use log::debug;
use object_type::ObjectType;
use typename_derive::TypeName;

use crate::{
    AssetLoadingResources, AssetPartLoader, AssetPartLoadingSystem, DefinitionLoadingResources,
};

/// Loads asset definitions.
pub type AssetDefinitionLoadingSystem = AssetPartLoadingSystem<AssetDefinitionLoader>;

/// `AssetDefinitionLoader`.
#[derive(Debug, TypeName)]
pub struct AssetDefinitionLoader;

impl<'s> AssetPartLoader<'s> for AssetDefinitionLoader {
    const LOAD_STAGE: LoadStage = LoadStage::AssetDefinitionLoading;
    type SystemData = DefinitionLoadingResources<'s>;

    fn process(
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
            .entry(LoadStage::AssetDefinitionLoading)
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

    fn is_complete(
        AssetLoadingResources {
            asset_type_mappings,
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
        asset_id: AssetId,
    ) -> bool {
        let asset_type = asset_type_mappings
            .get(asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        match asset_type {
            AssetType::Object(object_type) => match object_type {
                ObjectType::Character => asset_character_definition_handle
                    .get(asset_id)
                    .and_then(|character_definition_handle| {
                        character_definition_assets.get(character_definition_handle)
                    })
                    .is_some(),
                ObjectType::Energy => asset_energy_definition_handle
                    .get(asset_id)
                    .and_then(|character_definition_handle| {
                        energy_definition_assets.get(character_definition_handle)
                    })
                    .is_some(),
                ObjectType::TestObject => panic!("`TestObject` loading is not supported."),
            },
            AssetType::Map => asset_map_definition_handle
                .get(asset_id)
                .and_then(|map_definition_handle| map_definition_assets.get(map_definition_handle))
                .is_some(),
        }
    }
}
