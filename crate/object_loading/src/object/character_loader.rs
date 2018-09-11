use amethyst::{assets::Loader, prelude::*};
use application::{load_in, Format, Result};
use game_model::config::AssetRecord;
use object_model::{
    config::CharacterDefinition,
    loaded::{Character, CharacterHandle},
};

use object::ObjectLoader;

/// Loads `Character`s from configuration.
#[derive(Debug)]
pub struct CharacterLoader;

impl CharacterLoader {
    /// Returns the loaded `Character` model defined by character configuration.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load animations into.
    /// * `asset_record`: Entry of the object's configuration.
    pub fn load(world: &World, asset_record: &AssetRecord) -> Result<CharacterHandle> {
        let character_definition = load_in::<CharacterDefinition, _>(
            &asset_record.path,
            "object.toml",
            Format::Toml,
            None,
        )?;

        let object =
            ObjectLoader::load(world, asset_record, &character_definition.object_definition)?;
        let character = Character::new(object, character_definition);

        let loader = world.read_resource::<Loader>();
        let character_handle = loader.load_from_data(character, (), &world.read_resource());
        Ok(character_handle)
    }
}

#[cfg(test)]
mod test {
    use amethyst::assets::AssetStorage;
    use amethyst_test_support::prelude::*;
    use assets_test::{ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG};
    use game_model::config::AssetRecord;
    use object_model::loaded::{Character, CharacterHandle};

    use super::CharacterLoader;
    use ObjectLoadingBundle;

    #[test]
    fn loads_character() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_character", false)
                .with_bundle(ObjectLoadingBundle)
                .with_effect(|world| {
                    let asset_record = AssetRecord::new(
                        ASSETS_CHAR_BAT_SLUG.clone(),
                        ASSETS_CHAR_BAT_PATH.clone(),
                    );

                    let character_handle = CharacterLoader::load(world, &asset_record)
                        .expect("Failed to load character.");

                    world.add_resource(EffectReturn(character_handle));
                }).with_assertion(|world| {
                    let character_handle =
                        &world.read_resource::<EffectReturn<CharacterHandle>>().0;
                    let store = world.read_resource::<AssetStorage<Character>>();
                    assert!(store.get(character_handle).is_some());
                }).run()
                .is_ok()
        );
    }
}
