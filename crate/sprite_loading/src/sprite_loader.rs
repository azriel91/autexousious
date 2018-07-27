use std::path::Path;

use amethyst::{
    prelude::*,
    renderer::{MaterialTextureSet, SpriteSheet, TextureHandle},
};
use application::{load_in, Format, Result};
use sprite_model::{config::SpritesDefinition, loaded::SpriteMaterialMesh};

use MaterialCreator;
use SpriteMeshCreator;
use SpriteSheetMapper;
use TextureLoader;

/// Provides functionality to load sprites configuration and assets.
#[derive(Debug)]
pub struct SpriteLoader;

impl SpriteLoader {
    /// Loads and returns sprite assets for an object.
    ///
    /// The sprites base directory is expected to contain:
    ///
    /// * `sprites.toml`: Configuration file that defines what sprites to load.
    /// * Sprite sheets: The images that contain the sprites.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the loaded assets.
    /// * `texture_index_offset`: Index offset for sprite sheets and textures.
    /// * `base_dir`: Base directory from which to load sprites.
    pub fn load(
        world: &World,
        texture_index_offset: u64,
        base_dir: &Path,
    ) -> Result<(Vec<SpriteSheet>, SpriteMaterialMesh)> {
        let sprites_definition =
            load_in::<SpritesDefinition, _>(base_dir, "sprites.toml", Format::Toml, None)?;

        let sprite_sheets =
            SpriteSheetMapper::map(texture_index_offset, &sprites_definition.sheets);

        let mesh = SpriteMeshCreator::create_mesh(world, &sprites_definition);
        let mesh_mirrored = SpriteMeshCreator::create_mesh_mirrored(world, &sprites_definition);
        let texture_handles =
            TextureLoader::load_textures(world, base_dir, &sprites_definition.sheets)?;
        let default_material = MaterialCreator::create_default(world, &texture_handles);
        let sprite_material_mesh = SpriteMaterialMesh::new(default_material, mesh, mesh_mirrored);

        Self::store_textures_in_material_texture_set(world, texture_index_offset, texture_handles);

        Ok((sprite_sheets, sprite_material_mesh))
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
        texture_handles: Vec<TextureHandle>,
    ) {
        texture_handles
            .into_iter()
            .enumerate()
            .for_each(|(index, texture_handle)| {
                let texture_index = texture_index_offset + index as u64;
                debug!(
                    "Storing texture handle: `{:?}` in MaterialTextureSet index: `{:?}`",
                    texture_handle, texture_index
                );
                world
                    .write_resource::<MaterialTextureSet>()
                    .insert(texture_index, texture_handle);
            });
    }
}
