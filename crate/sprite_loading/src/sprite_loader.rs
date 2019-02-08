use std::path::Path;

use amethyst::{
    assets::{AssetStorage, Loader},
    renderer::{SpriteSheet, SpriteSheetHandle, Texture},
    Error,
};
use sprite_model::config::SpritesDefinition;

use crate::{SpriteSheetLoader, TextureLoader};

/// Provides functionality to load sprites configuration and assets.
#[derive(Debug)]
pub struct SpriteLoader;

impl SpriteLoader {
    /// Loads sprite sheet layout and texture data and returns their handles.
    ///
    /// The sprites base directory is expected to contain:
    ///
    /// * `sprites.toml`: Configuration file that defines what sprites to load.
    /// * Sprite sheets: The images that contain the sprites.
    ///
    /// # Parameters
    ///
    /// * `loader`: `Loader` to load assets.
    /// * `texture_assets`: `AssetStorage` for `Texture`s.
    /// * `sprite_sheet_assets`: `AssetStorage` for `SpriteSheet`s.
    /// * `sprites_definition`: The loaded `sprites.toml`.
    pub fn load(
        loader: &Loader,
        texture_assets: &AssetStorage<Texture>,
        sprite_sheet_assets: &AssetStorage<SpriteSheet>,
        sprites_definition: &SpritesDefinition,
        base_dir: &Path,
    ) -> Result<Vec<SpriteSheetHandle>, Error> {
        let texture_handles = TextureLoader::load_textures(
            loader,
            texture_assets,
            base_dir,
            &sprites_definition.sheets,
        )?;

        let sprite_sheet_handles = SpriteSheetLoader::load(
            loader,
            sprite_sheet_assets,
            &texture_handles,
            &sprites_definition.sheets,
        );

        Ok(sprite_sheet_handles)
    }
}
