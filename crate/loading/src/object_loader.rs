use std::fs::File;
use std::io::prelude::*;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{Material, MaterialDefaults, SpriteSheet, SpriteSheetHandle, TextureHandle};
use amethyst_animation::MaterialTextureSet;
use game_model::config::ConfigRecord;
use object_model::config::SpritesDefinition;
use object_model::loaded;
use toml;

use error::Result;
use sprite::into_sprite_sheet;
use texture;

pub struct ObjectLoader<'w> {
    /// The world in which to load object assets.
    pub world: &'w World,
    /// Offset for texture indices in the `MaterialTextureSet`
    texture_index_offset: usize,
}

impl<'w> ObjectLoader<'w> {
    pub fn new(world: &'w World) -> Self {
        ObjectLoader {
            world,
            texture_index_offset: 0,
        }
    }

    pub fn load_object(&mut self, config_record: &ConfigRecord) -> Result<loaded::Object> {
        let sprites_path = config_record.directory.join("sprites.toml");
        let mut sprites_toml = File::open(sprites_path)?;
        let mut buffer = Vec::new();
        sprites_toml.read_to_end(&mut buffer)?;

        let sprites_definition = toml::from_slice::<SpritesDefinition>(&buffer)?;

        let texture_index_offset = self.texture_index_offset;
        self.texture_index_offset += sprites_definition.sheets.len();

        let sprite_sheet_handles =
            self.load_sprite_sheets(&sprites_definition, texture_index_offset);
        let texture_handles = self.load_textures(sprites_definition);
        let default_material = self.create_default_material(&texture_handles);

        // TODO: Load animations.

        self.store_textures_in_material_texture_set(texture_handles, texture_index_offset);

        // TODO: Swap sprite_sheet_handles for animation handles
        Ok(loaded::Object::new(default_material, sprite_sheet_handles))
    }

    /// Computes the Amethyst sprite sheets and returns the handles to the sprite sheets.
    fn load_sprite_sheets(
        &self,
        sprites_definition: &SpritesDefinition,
        texture_index_offset: usize,
    ) -> Vec<SpriteSheetHandle> {
        sprites_definition
            .sheets
            .iter()
            .enumerate()
            .map(|(idx, definition)| into_sprite_sheet(texture_index_offset + idx, definition))
            .map(|sprite_sheet| {
                // Store the sprite sheet in asset storage.
                let loader = self.world.read_resource::<Loader>();
                loader.load_from_data(
                    sprite_sheet,
                    (),
                    &self.world.read_resource::<AssetStorage<SpriteSheet>>(),
                )
            })
            .collect::<Vec<SpriteSheetHandle>>()
    }

    /// Loads the sprite sheet images as textures and returns the texture handles.
    fn load_textures(&self, sprites_definition: SpritesDefinition) -> Vec<TextureHandle> {
        sprites_definition
            .sheets
            .into_iter()
            .map(|sheet_definition| texture::load(sheet_definition.path, &self.world))
            .collect::<Vec<TextureHandle>>()
    }

    /// Returns a material with the albedo set to the first sprite sheet texture.
    fn create_default_material(&self, texture_handles: &Vec<TextureHandle>) -> Material {
        let mat_defaults = self.world.read_resource::<MaterialDefaults>();
        texture_handles.first().map_or_else(
            || mat_defaults.0.clone(),
            |first_texture| Material {
                albedo: first_texture.clone(),
                ..mat_defaults.0.clone()
            },
        )
    }

    fn store_textures_in_material_texture_set(
        &self,
        texture_handles: Vec<TextureHandle>,
        texture_index_offset: usize,
    ) {
        texture_handles
            .into_iter()
            .enumerate()
            .for_each(|(index, texture_handle)| {
                self.world
                    .write_resource::<MaterialTextureSet>()
                    .insert(texture_index_offset + index, texture_handle);
            });
    }
}
