use amethyst::{
    assets::{AssetStorage, ProgressCounter},
    ecs::{Read, System, World, Write},
    renderer::{sprite::SpriteSheetHandle, SpriteSheet, Texture},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::AssetId;
use derivative::Derivative;
use derive_new::new;
use loading_model::loaded::{AssetLoadStatus, LoadStatus};
use log::debug;
use slotmap::SecondaryMap;
use sprite_loading::SpriteLoader;
use typename_derive::TypeName;

use crate::{AssetLoadingResources, SpriteLoadingResources};

/// Loads asset textures.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct AssetTextureLoadingSystem;

/// `AssetTextureLoadingSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetTextureLoadingSystemData<'s> {
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_load_status: Write<'s, AssetLoadStatus>,
    /// `AssetLoadingResources`.
    #[derivative(Debug = "ignore")]
    pub asset_loading_resources: AssetLoadingResources<'s>,
    /// `SpriteLoadingResources`.
    pub sprite_loading_resources: SpriteLoadingResources<'s>,
    /// `TextureLoadingResources`.
    pub texture_loading_resources: TextureLoadingResources<'s>,
}

/// `TextureLoadingResources`.
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

impl<'s> System<'s> for AssetTextureLoadingSystem {
    type SystemData = AssetTextureLoadingSystemData<'s>;

    fn run(
        &mut self,
        AssetTextureLoadingSystemData {
            mut asset_load_status,
            mut asset_loading_resources,
            sprite_loading_resources,
            mut texture_loading_resources,
        }: Self::SystemData,
    ) {
        asset_load_status
            .iter_mut()
            .filter(|(_, load_status)| **load_status == LoadStatus::SpritesLoading)
            .for_each(|(asset_id, load_status)| {
                if Self::sprites_definition_loaded(&sprite_loading_resources, asset_id) {
                    Self::texture_load(
                        &mut asset_loading_resources,
                        &sprite_loading_resources,
                        &mut texture_loading_resources,
                        asset_id,
                    );

                    *load_status = LoadStatus::TextureLoading;
                }
            });
    }
}

impl AssetTextureLoadingSystem {
    /// Returns whether the `SpritesDefinition` asset has been loaded.
    ///
    /// Returns `true` if there was no sprite definition for the asset.
    fn sprites_definition_loaded(
        SpriteLoadingResources {
            sprites_definition_assets,
            asset_sprites_definition_handles,
        }: &SpriteLoadingResources<'_>,
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
            asset_id_to_path,
            asset_id_mappings,
            load_status_progress_counters,
            loader,
            ..
        }: &mut AssetLoadingResources<'_>,
        SpriteLoadingResources {
            sprites_definition_assets,
            asset_sprites_definition_handles,
        }: &SpriteLoadingResources<'_>,
        TextureLoadingResources {
            texture_assets,
            sprite_sheet_assets,
            asset_sprite_sheet_handles,
        }: &mut TextureLoadingResources<'_>,
        asset_id: AssetId,
    ) {
        let mut progress_counter = load_status_progress_counters
            .entry(LoadStatus::TextureLoading)
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
}
