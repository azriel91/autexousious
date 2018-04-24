use amethyst::assets::{Handle, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{Material, SpriteSheet};
use amethyst_animation::Animation;
use object_model::config::CharacterDefinition;
use toml;

use animation::into_animation;
use error::Result;

/// Loads `Animation`s from character sequences.
#[derive(Debug)]
pub(super) struct CharacterAnimationLoader;

impl CharacterAnimationLoader {
    /// Loads `Animation`s from the object definition into the world, and returns their handles.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `texture_index_offset`: Offset of the texture IDs in the `MaterialTextureSet`.
    /// * `sprite_sheets`: `SpriteSheet`s that contain the texture coordinates for sprites.
    /// * `object_toml`: Contents of the object definition file.
    pub(super) fn load(
        world: &World,
        texture_index_offset: usize,
        sprite_sheets: &Vec<SpriteSheet>,
        object_toml: &Vec<u8>,
    ) -> Result<Vec<Handle<Animation<Material>>>> {
        let character_definition = toml::from_slice::<CharacterDefinition>(object_toml)?;
        let object_definition = character_definition.object_definition;
        let animation_handles = object_definition
            .sequences
            .iter()
            .map(|sequence| into_animation(world, texture_index_offset, sprite_sheets, sequence))
            .map(|animation| {
                let loader = world.read_resource::<Loader>();
                loader.load_from_data(animation, (), &world.read_resource())
            })
            .collect::<Vec<Handle<Animation<Material>>>>();

        Ok(animation_handles)
    }
}
