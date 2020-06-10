use amethyst::assets::ProgressCounter;
use asset_loading::YamlFormat;
use asset_model::{config::AssetType, loaded::AssetId};
use loading_model::loaded::LoadStage;
use loading_spi::{AssetLoadingResources, SpritesDefinitionLoadingResources};
use log::debug;
#[cfg(target_arch = "wasm32")]
use wasm_support_fs::PathAccessExt;

use crate::{AssetPartLoader, AssetPartLoadingSystem};

/// Loads asset sprites definitions.
pub type AssetSpritesDefinitionLoadingSystem = AssetPartLoadingSystem<AssetSpritesDefinitionLoader>;

/// `AssetSpritesDefinitionLoader`.
#[derive(Debug)]
pub struct AssetSpritesDefinitionLoader;

impl<'s> AssetPartLoader<'s> for AssetSpritesDefinitionLoader {
    const LOAD_STAGE: LoadStage = LoadStage::SpritesDefinitionLoading;
    type SystemData = SpritesDefinitionLoadingResources<'s>;

    /// Loads an asset's `SpritesDefinition`.
    fn process(
        AssetLoadingResources {
            asset_id_to_path,
            asset_id_mappings,
            asset_type_mappings,
            load_stage_progress_counters,
            loader,
        }: &mut AssetLoadingResources<'_>,
        SpritesDefinitionLoadingResources {
            sprites_definition_assets,
            asset_sprites_definition_handles,
        }: &mut SpritesDefinitionLoadingResources<'_>,
        asset_id: AssetId,
    ) {
        let asset_type = asset_type_mappings
            .get(asset_id)
            .expect("Expected `AssetType` mapping to exist.");

        let progress_counter = load_stage_progress_counters
            .entry(LoadStage::SpritesDefinitionLoading)
            .or_insert_with(ProgressCounter::new);

        let asset_slug = asset_id_mappings
            .slug(asset_id)
            .expect("Expected `AssetSlug` mapping to exist for `AssetId`.");
        let asset_path = asset_id_to_path
            .get(asset_id)
            .expect("Expected `PathBuf` mapping to exist for `AssetId`.");

        let sprites_definition_path = asset_path.join("sprites.yaml");
        if let AssetType::Map | AssetType::Ui = asset_type {
            // Return early if `sprites.yaml` does not exist.
            // This means `asset_sprites_definition_handles` will not have a key for the current
            // `asset_id`.
            #[cfg(not(target_arch = "wasm32"))]
            let sprites_definition_path_exists = sprites_definition_path.exists();
            #[cfg(target_arch = "wasm32")]
            let sprites_definition_path_exists = sprites_definition_path.exists_on_server();

            if !sprites_definition_path_exists {
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
    fn is_complete(
        AssetLoadingResources {
            asset_id_to_path,
            asset_type_mappings,
            ..
        }: &AssetLoadingResources<'_>,
        SpritesDefinitionLoadingResources {
            sprites_definition_assets,
            asset_sprites_definition_handles,
        }: &SpritesDefinitionLoadingResources<'_>,
        asset_id: AssetId,
    ) -> bool {
        asset_sprites_definition_handles
            .get(asset_id)
            .map(|sprites_definition_handle| {
                sprites_definition_assets
                    .get(sprites_definition_handle)
                    .is_some()
            })
            .unwrap_or_else(|| {
                let asset_type = asset_type_mappings
                    .get(asset_id)
                    .expect("Expected `AssetType` mapping to exist.");

                let asset_path = asset_id_to_path
                    .get(asset_id)
                    .expect("Expected `PathBuf` mapping to exist for `AssetId`.");

                let sprites_definition_path = asset_path.join("sprites.yaml");

                if let AssetType::Map | AssetType::Ui = asset_type {
                    // If there is no sprites definition, return `true`. Otherwise return `false`.
                    #[cfg(not(target_arch = "wasm32"))]
                    let sprites_definition_path_exists = sprites_definition_path.exists();
                    #[cfg(target_arch = "wasm32")]
                    let sprites_definition_path_exists = sprites_definition_path.exists_on_server();

                    !sprites_definition_path_exists
                } else {
                    false
                }
            })
    }
}
