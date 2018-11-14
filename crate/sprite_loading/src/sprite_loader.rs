use std::path::Path;

use amethyst::{
    prelude::*,
    renderer::{SpriteSheetHandle, TextureHandle},
};
use application::{load_in, Format, Result};
use sprite_model::config::SpritesDefinition;

use SpriteSheetLoader;
use TextureLoader;

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
    /// * `world`: `World` to store the loaded assets.
    /// * `base_dir`: Base directory from which to load sprites.
    pub fn load(
        world: &World,
        base_dir: &Path,
    ) -> Result<(Vec<SpriteSheetHandle>, Vec<TextureHandle>)> {
        let sprites_definition =
            load_in::<SpritesDefinition, _>(base_dir, "sprites.toml", Format::Toml, None)?;

        let texture_handles =
            TextureLoader::load_textures(world, base_dir, &sprites_definition.sheets)?;

        let sprite_sheet_handles =
            SpriteSheetLoader::load(world, &texture_handles, &sprites_definition.sheets);

        Ok((sprite_sheet_handles, texture_handles))
    }
}
