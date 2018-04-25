use amethyst::prelude::*;
use game_model::config::ConfigRecord;
use object_model::ObjectType;
use object_model::loaded;

use animation::AnimationLoader;
use error::Result;
use sprite::SpriteLoader;

/// Loads assets specified by object configuration into the loaded object model.
#[derive(Debug, Default, new)]
pub struct ObjectLoader {
    /// Offset for texture indices in the `MaterialTextureSet`
    #[new(default)]
    texture_index_offset: usize,
}

impl ObjectLoader {
    /// Returns the loaded `Object` referenced by the configuration record.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the object's assets.
    /// * `object_type`: Type of object, whether it is a character, weapon, etc.
    /// * `config_record`: Entry of the object's configuration.
    pub fn load_object(
        &mut self,
        world: &World,
        object_type: &ObjectType,
        config_record: &ConfigRecord,
    ) -> Result<loaded::Object> {
        let texture_index_offset = self.texture_index_offset;

        let (sprite_sheets, mesh, default_material) =
            SpriteLoader::load(world, texture_index_offset, config_record)?;

        let animation_handles = AnimationLoader::load(
            world,
            config_record,
            object_type,
            texture_index_offset,
            &sprite_sheets,
        )?;

        self.texture_index_offset += sprite_sheets.len();

        Ok(loaded::Object::new(
            default_material,
            mesh,
            animation_handles,
        ))
    }
}
