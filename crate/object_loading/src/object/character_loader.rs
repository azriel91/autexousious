use amethyst::prelude::*;
use game_model::config::ConfigRecord;
use object_model::config::CharacterDefinition;
use object_model::loaded::Character;
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
    /// * `object_loader`: Loader for common configuration data for all object types.
    pub fn load(
        world: &World,
        config_record: &ConfigRecord,
        object_loader: &mut ObjectLoader,
    ) -> Result<Character> {
        let object_toml = IoUtils::read_file(&config_record.directory.join("object.toml"))?;
        let character_definition = toml::from_slice::<CharacterDefinition>(&object_toml)?;

        let object = object_loader.load(
            world,
            config_record,
            &character_definition.object_definition,
        )?;

        Ok(Character::new(object, character_definition))
    }
}
