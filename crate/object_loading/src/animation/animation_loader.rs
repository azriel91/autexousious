use amethyst::assets::Handle;
use amethyst::prelude::*;
use amethyst::renderer::{Material, SpriteSheet};
use amethyst_animation::Animation;
use game_model::config::ConfigRecord;
use object_model::ObjectType;

use IoUtils;
use animation::CharacterAnimationLoader;
use error::Result;

#[derive(Debug)]
pub(crate) struct AnimationLoader;

impl AnimationLoader {
    /// Loads the object definition from the object configuration directory.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `config_record`: Entry of the object's configuration.
    /// * `object_type`: Type of object, whether it is a character, weapon, etc.
    /// * `texture_index_offset`: Offset of the texture IDs in the `MaterialTextureSet`.
    /// * `sprite_sheets`: `SpriteSheet`s that contain the texture coordinates for sprites.
    pub(crate) fn load(
        world: &World,
        config_record: &ConfigRecord,
        object_type: &ObjectType,
        texture_index_offset: usize,
        sprite_sheets: &[SpriteSheet],
    ) -> Result<Vec<Handle<Animation<Material>>>> {
        let object_toml = IoUtils::read_file(&config_record.directory.join("object.toml"))?;

        match *object_type {
            ObjectType::Character => CharacterAnimationLoader::load(
                world,
                texture_index_offset,
                sprite_sheets,
                &object_toml,
            ),
        }
    }
}
