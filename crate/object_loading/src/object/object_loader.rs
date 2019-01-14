use std::collections::HashMap;

use amethyst::{
    assets::{Handle, Loader},
    prelude::*,
    renderer::SpriteRender,
    Error,
};
use collision_loading::InteractionAnimationLoader;
use collision_model::{animation::InteractionFrameActiveHandle, config::InteractionFrame};
use fnv::FnvHashMap;
use game_model::config::AssetRecord;
use log::debug;
use object_model::{
    config::ObjectDefinition,
    loaded::{
        AnimatedComponentAnimation, AnimatedComponentDefault, GameObject, Object, ObjectWrapper,
        SequenceEndTransition, SequenceEndTransitions,
    },
};
use sprite_loading::{SpriteLoader, SpriteRenderAnimationLoader};

/// Loads assets specified by object configuration into the loaded object model.
#[derive(Debug)]
pub struct ObjectLoader;

impl ObjectLoader {
    /// Returns the loaded `Object` referenced by the asset record.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the object's assets.
    /// * `asset_record`: Entry of the object's configuration.
    /// * `object_definition`: Object definition configuration.
    pub fn load<O>(
        world: &World,
        asset_record: &AssetRecord,
        object_definition: &ObjectDefinition<O::SequenceId>,
    ) -> Result<
        (
            Handle<O::ObjectWrapper>,
            SequenceEndTransitions<O::SequenceId>,
        ),
        Error,
    >
    where
        O: GameObject,
    {
        debug!("Loading object assets in `{}`", asset_record.path.display());

        let sequence_end_transitions = object_definition
            .sequences
            .iter()
            .map(|(sequence_id, sequence)| {
                (*sequence_id, SequenceEndTransition::new(sequence.next))
            })
            .collect::<FnvHashMap<_, _>>();

        let (sprite_sheet_handles, _texture_handles) =
            SpriteLoader::load(world, &asset_record.path)?;

        // === Animation Component Defaults === //

        // Load the animation defaults in a separate scope because the animations' loaders may read
        // the `AnimationDataSet`s mutably, and that will cause a panic at runtime since loading
        // animation defaults borrows them immutably.

        let sprite_sheet = sprite_sheet_handles
            .iter()
            .next()
            .expect("Expected character to have at least one sprite sheet.")
            .clone();

        let animation_defaults = {
            let mut animation_defaults = Vec::new();

            let sprite_render = SpriteRender {
                sprite_sheet,
                sprite_number: 0,
            };
            animation_defaults.push(AnimatedComponentDefault::SpriteRender(sprite_render));

            let interaction_frame_handle = {
                let loader = world.read_resource::<Loader>();
                loader.load_from_data(InteractionFrame::default(), (), &world.read_resource())
            };
            let interaction_frame_active_handle =
                InteractionFrameActiveHandle::new(interaction_frame_handle);
            animation_defaults.push(AnimatedComponentDefault::InteractionFrame(
                interaction_frame_active_handle,
            ));

            animation_defaults
        };

        // === Animations === //

        let mut sprite_render_animations = SpriteRenderAnimationLoader::load_into_map(
            world,
            &object_definition.sequences,
            &sprite_sheet_handles,
        );
        let mut interaction_frame_animations =
            InteractionAnimationLoader::load_into_map(world, &object_definition.sequences);

        let animations = object_definition
            .sequences
            .keys()
            .map(move |sequence_id| {
                let mut animations = Vec::new();
                if let Some(sprite_render) = sprite_render_animations.remove(sequence_id) {
                    animations.push(AnimatedComponentAnimation::SpriteRender(sprite_render));
                }
                if let Some(interaction_frame) = interaction_frame_animations.remove(sequence_id) {
                    animations.push(AnimatedComponentAnimation::InteractionFrame(
                        interaction_frame,
                    ));
                }

                (*sequence_id, animations)
            })
            .collect::<HashMap<_, _>>();

        let object = Object::new(animation_defaults, animations);
        let object_wrapper = O::ObjectWrapper::new(object);
        let object_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load_from_data(object_wrapper, (), &world.read_resource())
        };

        Ok((object_handle, sequence_end_transitions.into()))
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        animation::AnimationBundle,
        assets::{AssetStorage, Handle},
    };
    use amethyst_test::AmethystApplication;
    use application::{load_in, Format};
    use assets_test::{ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG};
    use character_model::{
        config::{CharacterDefinition, CharacterSequenceId},
        loaded::{Character, CharacterObjectWrapper},
    };
    use collision_loading::CollisionLoadingBundle;
    use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
    use game_model::config::AssetRecord;

    use super::ObjectLoader;
    use crate::ObjectLoadingBundle;

    #[test]
    fn loads_object_assets() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_object_assets", false)
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

                    let character_definition = load_in::<CharacterDefinition, _>(
                        &asset_record.path,
                        "object.toml",
                        Format::Toml,
                        None,
                    )
                    .expect("Failed to load object.toml into CharacterDefinition");

                    let (object_handle, _sequence_end_transitions) =
                        ObjectLoader::load::<Character>(
                            world,
                            &asset_record,
                            &character_definition.object_definition,
                        )
                        .expect("Failed to load object");

                    world.add_resource(object_handle);
                })
                .with_assertion(|world| {
                    let object_handle = world.read_resource::<Handle<CharacterObjectWrapper>>();
                    let object_assets =
                        world.read_resource::<AssetStorage<CharacterObjectWrapper>>();
                    let object = object_assets
                        .get(&object_handle)
                        .expect("Expected object to be loaded after one tick.");

                    // See bat/object.toml
                    assert_eq!(16, object.animations.len());

                    let stand_attack_animations = object
                        .animations
                        .get(&CharacterSequenceId::StandAttack)
                        .expect("Expected to read `StandAttack` animations.");
                    // InteractionFrame and SpriteRender
                    assert_eq!(2, stand_attack_animations.len());
                })
                .run()
                .is_ok()
        );
    }
}
