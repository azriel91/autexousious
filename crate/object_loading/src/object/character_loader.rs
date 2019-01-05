use amethyst::{assets::Loader, prelude::*};
use application::{load_in, Format, Result};
use character_model::{
    config::CharacterDefinition,
    loaded::{Character, CharacterHandle},
};
use game_model::config::AssetRecord;

use crate::object::ObjectLoader;

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

        let (object_handle, sequence_end_transitions) = ObjectLoader::load::<Character>(
            world,
            asset_record,
            &character_definition.object_definition,
        )?;
        let character = Character::new(object_handle, sequence_end_transitions);

        let loader = world.read_resource::<Loader>();
        let character_handle = loader.load_from_data(character, (), &world.read_resource());
        Ok(character_handle)
    }
}

#[cfg(test)]
mod test {
    use amethyst::{animation::AnimationBundle, assets::AssetStorage};
    use amethyst_test::prelude::*;
    use assets_test::{ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG};
    use character_model::config::CharacterSequenceId;
    use character_model::loaded::{Character, CharacterHandle};
    use collision_loading::CollisionLoadingBundle;
    use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
    use game_model::config::AssetRecord;

    use super::CharacterLoader;
    use crate::ObjectLoadingBundle;

    #[test]
    fn loads_character() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_character", false)
                .with_bundle(
                    AnimationBundle::<CharacterSequenceId, BodyFrameActiveHandle>::new(
                        "character_body_frame_acs",
                        "character_body_frame_sis",
                    )
                )
                .with_bundle(AnimationBundle::<
                    CharacterSequenceId,
                    InteractionFrameActiveHandle,
                >::new(
                    "character_interaction_frame_acs",
                    "character_interaction_frame_sis",
                ))
                .with_bundle(CollisionLoadingBundle::new())
                .with_bundle(ObjectLoadingBundle::new())
                .with_effect(|world| {
                    let asset_record = AssetRecord::new(
                        ASSETS_CHAR_BAT_SLUG.clone(),
                        ASSETS_CHAR_BAT_PATH.clone(),
                    );

                    let character_handle = CharacterLoader::load(world, &asset_record)
                        .expect("Failed to load character.");

                    world.add_resource(EffectReturn(character_handle));
                })
                .with_assertion(|world| {
                    let character_handle =
                        &world.read_resource::<EffectReturn<CharacterHandle>>().0;
                    let store = world.read_resource::<AssetStorage<Character>>();
                    assert!(store.get(character_handle).is_some());
                })
                .run()
                .is_ok()
        );
    }
}
