use std::path::Path;

use amethyst::{
    prelude::*,
    renderer::{MaterialTextureSet, SpriteSheetHandle, TextureHandle},
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
    /// * `sprite_sheet_index_offset`: Index offset for sprite sheets and textures.
    /// * `base_dir`: Base directory from which to load sprites.
    pub fn load(
        world: &World,
        sprite_sheet_index_offset: u64,
        base_dir: &Path,
    ) -> Result<(Vec<SpriteSheetHandle>, Vec<TextureHandle>)> {
        let sprites_definition =
            load_in::<SpritesDefinition, _>(base_dir, "sprites.toml", Format::Toml, None)?;

        let sprite_sheet_handles =
            SpriteSheetLoader::load(world, sprite_sheet_index_offset, &sprites_definition.sheets);

        let texture_handles =
            TextureLoader::load_textures(world, base_dir, &sprites_definition.sheets)?;
        Self::store_textures_in_material_texture_set(
            world,
            sprite_sheet_index_offset,
            &texture_handles,
        );

        Ok((sprite_sheet_handles, texture_handles))
    }

    /// Stores the texture handles into the global `MaterialTextureSet`.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the texture handles.
    /// * `texture_index_offset`: The texture index offset to begin with.
    /// * `texture_handles`: Texture handles to store.
    fn store_textures_in_material_texture_set(
        world: &World,
        texture_index_offset: u64,
        texture_handles: &[TextureHandle],
    ) {
        texture_handles
            .iter()
            .enumerate()
            .for_each(|(index, texture_handle)| {
                let texture_index = texture_index_offset + index as u64;
                debug!(
                    "Storing texture handle: `{:?}` in MaterialTextureSet index: `{:?}`",
                    texture_handle, texture_index
                );
                world
                    .write_resource::<MaterialTextureSet>()
                    .insert(texture_index, texture_handle.clone());
            });
    }
}
