use amethyst::prelude::*;
use amethyst::renderer::{Material, MeshHandle, SpriteSheet, TextureHandle};
use amethyst_animation::MaterialTextureSet;
use game_model::config::ConfigRecord;
use object_model::config::SpritesDefinition;
use toml;

use IoUtils;
use error::Result;
use sprite::{MaterialCreator, SpriteMeshCreator, SpriteSheetMapper, TextureLoader};

/// Provides functionality to load sprites configuration and assets.
#[derive(Debug)]
pub(crate) struct SpriteLoader;

impl SpriteLoader {
    /// Loads and returns sprite assets for an object.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the loaded assets.
    /// * `texture_index_offset`: Index offset for sprite sheets and textures.
    /// * `config_record`: Configuration record of the object for which to load sprites.
    pub fn load(
        world: &World,
        texture_index_offset: usize,
        config_record: &ConfigRecord,
    ) -> Result<(Vec<SpriteSheet>, MeshHandle, Material)> {
        let sprites_definition = Self::load_sprites_definition(&config_record)?;

        let sprite_sheets =
            SpriteSheetMapper::map(texture_index_offset, &sprites_definition.sheets);
        let mesh = SpriteMeshCreator::create_mesh(world, &sprites_definition);
        let texture_handles = TextureLoader::load_textures(
            world,
            &config_record.directory,
            &sprites_definition.sheets,
        );

        let default_material = MaterialCreator::create_default(world, &texture_handles);

        Self::store_textures_in_material_texture_set(world, texture_index_offset, texture_handles);

        Ok((sprite_sheets, mesh, default_material))
    }

    /// Loads the sprites definition from the object configuration directory.
    ///
    /// # Parameters
    ///
    /// * `config_record`: the configuration record of the object to load sprites for.
    fn load_sprites_definition(config_record: &ConfigRecord) -> Result<SpritesDefinition> {
        let file_path = config_record.directory.join("sprites.toml");
        let sprites_toml = IoUtils::read_file(&file_path)?;
        Ok(toml::from_slice::<SpritesDefinition>(&sprites_toml)?)
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
        texture_index_offset: usize,
        texture_handles: Vec<TextureHandle>,
    ) {
        texture_handles
            .into_iter()
            .enumerate()
            .for_each(|(index, texture_handle)| {
                let texture_index = texture_index_offset + index;
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
