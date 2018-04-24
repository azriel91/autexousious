use std::fs::File;
use std::io::prelude::*;

use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{Material, MaterialDefaults, Mesh, MeshHandle, PosTex, SpriteSheet,
                         TextureHandle};
use amethyst_animation::{Animation, InterpolationFunction, MaterialChannel, MaterialPrimitive,
                         MaterialTextureSet, Sampler};
use game_model::config::ConfigRecord;
use object_model::ObjectType;
use object_model::config::object::Sequence;
use object_model::config::{CharacterDefinition, SpritesDefinition};
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

    pub fn load_object(
        &mut self,
        object_type: &ObjectType,
        config_record: &ConfigRecord,
    ) -> Result<loaded::Object> {
        let sprites_definition = self.load_sprites_definition(&config_record)?;

        let texture_index_offset = self.texture_index_offset;
        self.texture_index_offset += sprites_definition.sheets.len();

        let sprite_sheets = self.load_sprite_sheets(&sprites_definition, texture_index_offset);
        let mesh = self.create_mesh(&sprites_definition);
        let texture_handles = self.load_textures(&config_record, &sprites_definition);
        let default_material = self.create_default_material(&texture_handles);
        let animation_handles = self.load_animations(
            &config_record,
            &object_type,
            texture_index_offset,
            &sprite_sheets,
        )?;

        self.store_textures_in_material_texture_set(texture_handles, texture_index_offset);

        Ok(loaded::Object::new(
            default_material,
            mesh,
            animation_handles,
        ))
    }

    /// Loads the sprites definition from the object configuration directory.
    fn load_sprites_definition(&self, config_record: &ConfigRecord) -> Result<SpritesDefinition> {
        let sprites_toml = Self::read_file(config_record, "sprites.toml")?;
        Ok(toml::from_slice::<SpritesDefinition>(&sprites_toml)?)
    }

    /// Computes the Amethyst sprite sheets and returns the handles to the sprite sheets.
    fn load_sprite_sheets(
        &self,
        sprites_definition: &SpritesDefinition,
        texture_index_offset: usize,
    ) -> Vec<SpriteSheet> {
        sprites_definition
            .sheets
            .iter()
            .enumerate()
            .map(|(idx, definition)| into_sprite_sheet(texture_index_offset + idx, definition))
            .collect::<Vec<SpriteSheet>>()
    }

    /// Creates a mesh for mapping the object's sprites to screen.
    fn create_mesh(&self, sprites_definition: &SpritesDefinition) -> MeshHandle {
        let (sprite_w, sprite_h) = {
            sprites_definition
                .sheets
                .first()
                .map_or((1., 1.), |sheet_def| {
                    (sheet_def.sprite_w, sheet_def.sprite_h)
                })
        };

        let loader = self.world.read_resource::<Loader>();
        loader.load_from_data(
            Self::create_mesh_vertices(sprite_w, sprite_h).into(),
            (),
            &self.world.read_resource::<AssetStorage<Mesh>>(),
        )
    }

    /// Returns a set of vertices that make up a rectangular mesh of the given size.
    ///
    /// This function expects pixel coordinates -- starting from the top left of the image. X increases
    /// to the right, Y increases downwards.
    ///
    /// # Parameters
    ///
    /// * `sprite_w`: Width of each sprite, excluding the border pixel if any.
    /// * `sprite_h`: Height of each sprite, excluding the border pixel if any.
    fn create_mesh_vertices(sprite_w: f32, sprite_h: f32) -> Vec<PosTex> {
        vec![
            PosTex {
                position: [0., 0., 0.],
                tex_coord: [0., 0.],
            },
            PosTex {
                position: [sprite_w, 0., 0.],
                tex_coord: [1., 0.],
            },
            PosTex {
                position: [0., sprite_h, 0.],
                tex_coord: [0., 1.],
            },
            PosTex {
                position: [sprite_w, sprite_h, 0.],
                tex_coord: [1., 1.],
            },
            PosTex {
                position: [0., sprite_h, 0.],
                tex_coord: [0., 1.],
            },
            PosTex {
                position: [sprite_w, 0., 0.],
                tex_coord: [1., 0.],
            },
        ]
    }

    /// Loads the sprite sheet images as textures and returns the texture handles.
    fn load_textures(
        &self,
        config_record: &ConfigRecord,
        sprites_definition: &SpritesDefinition,
    ) -> Vec<TextureHandle> {
        sprites_definition
            .sheets
            .iter()
            .map(|sheet_definition| {
                texture::load(
                    // TODO: resilient code
                    String::from(
                        config_record
                            .directory
                            .join(&sheet_definition.path)
                            .to_str()
                            .unwrap(),
                    ),
                    &self.world,
                )
            })
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

    /// Stores the texture handles into the global `MaterialTextureSet`.
    ///
    /// # Parameters
    ///
    /// * `texture_handles`: Texture handles to store.
    /// * `texture_index_offset`: The texture index offset to begin with.
    fn store_textures_in_material_texture_set(
        &self,
        texture_handles: Vec<TextureHandle>,
        texture_index_offset: usize,
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
                self.world
                    .write_resource::<MaterialTextureSet>()
                    .insert(texture_index, texture_handle);
            });
    }

    /// Loads the object definition from the object configuration directory.
    fn load_animations(
        &self,
        config_record: &ConfigRecord,
        object_type: &ObjectType,
        texture_index_offset: usize,
        sprite_sheets: &Vec<SpriteSheet>,
    ) -> Result<Vec<Handle<Animation<Material>>>> {
        let object_toml = Self::read_file(config_record, "object.toml")?;

        match *object_type {
            ObjectType::Character => {
                self.load_character_animations(object_toml, texture_index_offset, sprite_sheets)
            }
        }
    }

    fn read_file<'f>(config_record: &ConfigRecord, file_name: &'f str) -> Result<Vec<u8>> {
        let object_path = config_record.directory.join(file_name);
        let mut file = File::open(object_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn load_character_animations(
        &self,
        object_toml: Vec<u8>,
        texture_index_offset: usize,
        sprite_sheets: &Vec<SpriteSheet>,
    ) -> Result<Vec<Handle<Animation<Material>>>> {
        let character_definition = toml::from_slice::<CharacterDefinition>(&object_toml)?;
        let object_definition = character_definition.object_definition;
        let animation_handles = object_definition
            .sequences
            .iter()
            .map(|sequence| self.into_animation(sequence, texture_index_offset, sprite_sheets))
            .map(|animation| {
                let loader = self.world.read_resource::<Loader>();
                loader.load_from_data(animation, (), &self.world.read_resource())
            })
            .collect::<Vec<Handle<Animation<Material>>>>();

        Ok(animation_handles)
    }

    fn into_animation<SeqId>(
        &self,
        sequence: &Sequence<SeqId>,
        texture_index_offset: usize,
        sprite_sheets: &Vec<SpriteSheet>,
    ) -> Animation<Material> {
        let mut input = Vec::with_capacity(sequence.frames.len() + 1);
        let mut tick_counter = 0.;
        for frame in sequence.frames.iter() {
            input.push(tick_counter);
            tick_counter += 1. + frame.wait as f32;
        }
        input.push(tick_counter);

        let texture_sampler = Self::texture_sampler(sequence, texture_index_offset, input.clone());
        let sprite_offset_sampler = Self::sprite_offset_sampler(sequence, sprite_sheets, input);

        let loader = self.world.read_resource::<Loader>();
        let texture_animation_handle =
            loader.load_from_data(texture_sampler, (), &self.world.read_resource());
        let sampler_animation_handle =
            loader.load_from_data(sprite_offset_sampler, (), &self.world.read_resource());

        Animation {
            nodes: vec![
                (0, MaterialChannel::AlbedoTexture, texture_animation_handle),
                (0, MaterialChannel::AlbedoOffset, sampler_animation_handle),
            ],
        }
    }

    fn texture_sampler<SeqId>(
        sequence: &Sequence<SeqId>,
        texture_index_offset: usize,
        input: Vec<f32>,
    ) -> Sampler<MaterialPrimitive> {
        let mut output = sequence
            .frames
            .iter()
            .map(|frame| MaterialPrimitive::Texture(texture_index_offset + frame.sheet as usize))
            .collect::<Vec<MaterialPrimitive>>();
        let final_key_frame = {
            let last_frame = output.last();
            if last_frame.is_some() {
                Some(last_frame.unwrap().clone())
            } else {
                None
            }
        };
        if final_key_frame.is_some() {
            output.push(final_key_frame.unwrap());
        }

        Sampler {
            input,
            output,
            function: InterpolationFunction::Step,
        }
    }

    fn sprite_offset_sampler<SeqId>(
        sequence: &Sequence<SeqId>,
        sprite_sheets: &Vec<SpriteSheet>,
        input: Vec<f32>,
    ) -> Sampler<MaterialPrimitive> {
        let mut output = sequence
            .frames
            .iter()
            .map(|frame| (&sprite_sheets[frame.sheet].sprites[frame.sprite]).into())
            .collect::<Vec<MaterialPrimitive>>();
        let final_key_frame = {
            let last_frame = output.last();
            if last_frame.is_some() {
                Some(last_frame.unwrap().clone())
            } else {
                None
            }
        };
        if final_key_frame.is_some() {
            output.push(final_key_frame.unwrap());
        }

        Sampler {
            input,
            output,
            function: InterpolationFunction::Step,
        }
    }
}
