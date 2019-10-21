use amethyst::assets::ProgressCounter;
use asset_model::loaded::AssetId;
use loading_model::loaded::LoadStage;
use log::debug;
use sprite_loading::SpriteLoader;
use typename_derive::TypeName;

use crate::{
    AssetLoadingResources, AssetPartLoader, AssetPartLoadingSystem,
    SpritesDefinitionLoadingResourcesRead, TextureLoadingResources,
};

/// Loads asset sprites definitions.
pub type AssetTextureLoadingSystem = AssetPartLoadingSystem<AssetTextureLoader>;

/// `AssetTextureLoader`.
#[derive(Debug, TypeName)]
pub struct AssetTextureLoader;

impl<'s> AssetPartLoader<'s> for AssetTextureLoader {
    const LOAD_STAGE: LoadStage = LoadStage::TextureLoading;
    type SystemData = TextureLoadingResources<'s>;

    /// Loads an asset's `Texture`s and `SpriteSheet`s.
    fn process(
        AssetLoadingResources {
            asset_id_to_path,
            asset_id_mappings,
            load_stage_progress_counters,
            loader,
            ..
        }: &mut AssetLoadingResources<'_>,
        TextureLoadingResources {
            sprites_definition_loading_resources_read:
                SpritesDefinitionLoadingResourcesRead {
                    sprites_definition_assets,
                    asset_sprites_definition_handles,
                },
            texture_assets,
            sprite_sheet_assets,
            asset_sprite_sheet_handles,
        }: &mut TextureLoadingResources<'_>,
        asset_id: AssetId,
    ) {
        let mut progress_counter = load_stage_progress_counters
            .entry(LoadStage::TextureLoading)
            .or_insert_with(ProgressCounter::new);

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
    fn is_complete(
        _: &AssetLoadingResources<'_>,
        TextureLoadingResources {
            texture_assets,
            sprite_sheet_assets,
            asset_sprite_sheet_handles,
            ..
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
}
