use amethyst::{
    animation::{Animation, Sampler, SpriteRenderPrimitive},
    assets::{AssetStorage, Loader},
    ecs::World,
    renderer::{SpriteRender, SpriteSheet, Texture},
    Error,
};
use application::{load_in, Format};
use character_model::{
    config::CharacterDefinition,
    loaded::{Character, CharacterHandle},
};
use collision_model::{
    animation::{InteractionFrameActiveHandle, InteractionFramePrimitive},
    config::InteractionFrame,
};
use game_model::config::AssetRecord;

use crate::object::{ObjectLoader, ObjectLoaderParams};

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
    pub fn load(world: &World, asset_record: &AssetRecord) -> Result<CharacterHandle, Error> {
        let character_definition = load_in::<CharacterDefinition, _>(
            &asset_record.path,
            "object.toml",
            Format::Toml,
            None,
        )?;

        // TODO: Put sequence_end_transitions in `Object`
        let (_object_wrapper, sequence_end_transitions) = {
            let loader = &world.read_resource::<Loader>();
            let texture_assets = &world.read_resource::<AssetStorage<Texture>>();
            let sprite_sheet_assets = &world.read_resource::<AssetStorage<SpriteSheet>>();
            let sprite_render_primitive_sampler_assets =
                &world.read_resource::<AssetStorage<Sampler<SpriteRenderPrimitive>>>();
            let sprite_render_animation_assets =
                &world.read_resource::<AssetStorage<Animation<SpriteRender>>>();
            let interaction_frame_assets = &world.read_resource::<AssetStorage<InteractionFrame>>();
            let interaction_frame_primitive_sampler_assets =
                &world.read_resource::<AssetStorage<Sampler<InteractionFramePrimitive>>>();
            let interaction_frame_animation_assets =
                &world.read_resource::<AssetStorage<Animation<InteractionFrameActiveHandle>>>();

            ObjectLoader::load::<Character>(
                ObjectLoaderParams {
                    loader,
                    texture_assets,
                    sprite_sheet_assets,
                    sprite_render_primitive_sampler_assets,
                    sprite_render_animation_assets,
                    interaction_frame_assets,
                    interaction_frame_primitive_sampler_assets,
                    interaction_frame_animation_assets,
                },
                asset_record,
                &character_definition.object_definition,
            )?
        };

        let loader = world.read_resource::<Loader>();
        let definition_handle =
            loader.load_from_data(character_definition, (), &world.read_resource());
        let wrapper_handle = loader.load_from_data(definition_handle, (), &world.read_resource());
        let character = Character::new(wrapper_handle, sequence_end_transitions);

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
    use character_model::{
        config::CharacterSequenceId,
        loaded::{Character, CharacterHandle},
    };
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
