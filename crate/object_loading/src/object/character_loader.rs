use amethyst::{assets::Loader, prelude::*};
use game_model::config::ConfigRecord;
use object_model::{
    config::CharacterDefinition,
    loaded::{Character, CharacterHandle},
};
use toml;

use error::Result;
use object::ObjectLoader;
use IoUtils;

/// Loads `Character`s from configuration.
#[derive(Debug)]
pub struct CharacterLoader;

impl CharacterLoader {
    /// Returns the loaded `Character` model defined by character configuration
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `config_record`: Entry of the object's configuration.
    pub fn load(world: &World, config_record: &ConfigRecord) -> Result<CharacterHandle> {
        let object_toml = IoUtils::read_file(&config_record.directory.join("object.toml"))?;
        let character_definition = toml::from_slice::<CharacterDefinition>(&object_toml)?;

        let object = ObjectLoader::load(
            world,
            config_record,
            &character_definition.object_definition,
        )?;
        let character = Character::new(object, character_definition);

        let loader = world.read_resource::<Loader>();
        let character_handle = loader.load_from_data(character, (), &world.read_resource());
        Ok(character_handle)
    }
}
