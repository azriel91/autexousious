use amethyst::{
    animation::get_animation_set,
    core::transform::Transform,
    ecs::Entity,
    renderer::{Flipped, SpriteRender, Transparent},
};
use animation_support::AnimationRunner;
use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
use object_model::{
    entity::{Mirrored, Position, SequenceStatus, Velocity},
    loaded::{AnimatedComponentAnimation, AnimatedComponentDefault, ObjectWrapper},
};

use crate::{ObjectAnimationStorages, ObjectComponentStorages};

/// Augments an entity with `Object` components.
#[derive(Debug)]
pub struct ObjectEntityAugmenter;

impl ObjectEntityAugmenter {
    /// Augments an entity with `Object` components.
    ///
    /// # Parameters
    ///
    /// * `entity`: The entity to augment.
    /// * `object_component_storages`: Common `Component` storages for objects.
    /// * `object_animation_storages`: Common animation storages for objects.
    /// * `object_wrapper`: Slug and handle of the object to spawn.
    pub fn augment<'s, W>(
        entity: Entity,
        ObjectComponentStorages {
            ref mut sprite_renders,
            ref mut flippeds,
            ref mut transparents,
            ref mut positions,
            ref mut velocities,
            ref mut transforms,
            ref mut mirroreds,
            ref mut sequence_end_transitionses,
            ref mut sequence_ids,
            ref mut sequence_statuses,
            ref mut body_frame_active_handles,
            ref mut interaction_frame_active_handles,
        }: &mut ObjectComponentStorages<'s, W::SequenceId>,
        ObjectAnimationStorages {
            ref mut sprite_render_acses,
            ref mut body_acses,
            ref mut interaction_acses,
        }: &mut ObjectAnimationStorages<'s, W::SequenceId>,
        object_wrapper: &W,
    ) where
        W: ObjectWrapper,
    {
        let sequence_end_transitions = &object_wrapper.inner().sequence_end_transitions;

        let sequence_id = W::SequenceId::default();

        flippeds
            .insert(entity, Flipped::None)
            .expect("Failed to insert flipped component.");
        transparents
            .insert(entity, Transparent)
            .expect("Failed to insert transparent component.");
        positions
            .insert(entity, Position::default())
            .expect("Failed to insert position component.");
        velocities
            .insert(entity, Velocity::default())
            .expect("Failed to insert velocity component.");
        transforms
            .insert(entity, Transform::default())
            .expect("Failed to insert transform component.");
        mirroreds
            .insert(entity, Mirrored::default())
            .expect("Failed to insert mirrored component.");
        sequence_end_transitionses
            .insert(entity, sequence_end_transitions.clone())
            .expect("Failed to insert sequence_end_transitions component.");
        sequence_ids
            .insert(entity, sequence_id)
            .expect("Failed to insert sequence_id component.");
        sequence_statuses
            .insert(entity, SequenceStatus::default())
            .expect("Failed to insert sequence_status component.");

        let all_animations = object_wrapper.inner().animations.get(&sequence_id);
        let animation_defaults = &object_wrapper.inner().animation_defaults;
        let first_sequence_animations = all_animations
            .as_ref()
            .expect("Expected game object to have at least one sequence.");

        animation_defaults
            .iter()
            .for_each(|animation_default| match animation_default {
                AnimatedComponentDefault::SpriteRender(ref sprite_render) => {
                    // The starting pose
                    sprite_renders
                        .insert(entity, sprite_render.clone())
                        .expect("Failed to insert `SpriteRender` component.");
                }
                AnimatedComponentDefault::BodyFrame(ref active_handle) => {
                    // Default body active handle
                    body_frame_active_handles
                        .insert(entity, active_handle.clone())
                        .expect("Failed to insert `BodyFrameActiveHandle` component.");
                }
                AnimatedComponentDefault::InteractionFrame(ref active_handle) => {
                    // Default interaction active handle
                    interaction_frame_active_handles
                        .insert(entity, active_handle.clone())
                        .expect("Failed to insert `InteractionFrameActiveHandle` component.");
                }
            });

        // We also need to trigger the animation, not just attach it to the entity
        let mut sprite_animation_set =
            get_animation_set::<W::SequenceId, SpriteRender>(sprite_render_acses, entity)
                .expect("Sprite animation should exist as new entity should be valid.");
        let mut body_animation_set =
            get_animation_set::<W::SequenceId, BodyFrameActiveHandle>(body_acses, entity)
                .expect("Body animation should exist as new entity should be valid.");
        let mut interaction_animation_set = get_animation_set::<
            W::SequenceId,
            InteractionFrameActiveHandle,
        >(interaction_acses, entity)
        .expect("Interaction animation should exist as new entity should be valid.");

        first_sequence_animations
            .iter()
            .for_each(|animated_component| match animated_component {
                AnimatedComponentAnimation::SpriteRender(ref handle) => {
                    AnimationRunner::start(sequence_id, &mut sprite_animation_set, handle);
                }
                AnimatedComponentAnimation::BodyFrame(ref handle) => {
                    AnimationRunner::start(sequence_id, &mut body_animation_set, handle);
                }
                AnimatedComponentAnimation::InteractionFrame(ref handle) => {
                    AnimationRunner::start(sequence_id, &mut interaction_animation_set, handle);
                }
            });
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{
        animation::AnimationBundle,
        assets::AssetStorage,
        core::transform::Transform,
        ecs::{Builder, Entity, Read, System, SystemData, World},
        renderer::{Flipped, SpriteRender, Transparent},
    };
    use amethyst_test::prelude::*;
    use application_event::{AppEvent, AppEventReader};
    use assets_test::{ASSETS_CHAR_BAT_SLUG, ASSETS_PATH};
    use character_loading::{CharacterComponentStorages, CharacterLoadingBundle};
    use character_model::{
        config::CharacterSequenceId,
        loaded::{Character, CharacterObjectWrapper},
    };
    use collision_loading::CollisionLoadingBundle;
    use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
    use game_model::loaded::SlugAndHandle;
    use loading::LoadingState;
    use map_loading::MapLoadingBundle;
    use map_model::loaded::Map;
    use object_model::{
        entity::{Mirrored, Position, SequenceStatus, Velocity},
        loaded::GameObject,
    };
    use typename::TypeName as TypeNameTrait;
    use typename_derive::TypeName;

    use super::ObjectEntityAugmenter;
    use crate::{ObjectAnimationStorages, ObjectComponentStorages};

    #[test]
    fn augments_entity_with_object_components() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        let setup = |world: &mut World| {
            let entity = world.create_entity().build();
            {
                let slug_and_handle =
                    SlugAndHandle::<Character>::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                let game_object_assets = world.read_resource::<AssetStorage<Character>>();
                let game_object = game_object_assets
                    .get(&slug_and_handle.handle)
                    .expect("Expected bat character to be loaded.");
                let object_wrapper_handle = game_object.object_handle();

                let object_wrapper_assets =
                    world.read_resource::<AssetStorage<CharacterObjectWrapper>>();
                let object_wrapper = object_wrapper_assets
                    .get(object_wrapper_handle)
                    .expect("Expected bat object to be loaded.");

                let mut object_component_storages = ObjectComponentStorages::fetch(&world.res);
                let mut object_animation_storages = ObjectAnimationStorages::fetch(&world.res);
                ObjectEntityAugmenter::augment(
                    entity,
                    &mut object_component_storages,
                    &mut object_animation_storages,
                    object_wrapper,
                );
            }

            world.add_resource(EffectReturn(entity));
        };

        let assertion = |world: &mut World| {
            let entity = world.read_resource::<EffectReturn<Entity>>().0;
            assert!(world.read_storage::<CharacterSequenceId>().contains(entity));
            assert!(world.read_storage::<SequenceStatus>().contains(entity));
            assert!(world.read_storage::<Mirrored>().contains(entity));
            assert!(world.read_storage::<SpriteRender>().contains(entity));
            assert!(world.read_storage::<Flipped>().contains(entity));
            assert!(world.read_storage::<Transparent>().contains(entity));
            assert!(world.read_storage::<Position<f32>>().contains(entity));
            assert!(world.read_storage::<Velocity<f32>>().contains(entity));
            assert!(world.read_storage::<Transform>().contains(entity));
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("augments_entity_with_object_components", false)
                .with_custom_event_type::<AppEvent, AppEventReader>()
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
                    "character_interaction_acs", "character_interaction_sis",
                ))
                .with_bundle(CollisionLoadingBundle::new())
                .with_bundle(MapLoadingBundle::new())
                .with_bundle(CharacterLoadingBundle::new())
                .with_system(TestSystem, TestSystem::type_name(), &[])
                .with_state(|| LoadingState::new(ASSETS_PATH.clone(), PopState))
                .with_setup(setup)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }

    // Sets up storages for the various `Component`.
    #[derive(Debug, TypeName)]
    struct TestSystem;
    type TestSystemData<'s> = (
        CharacterComponentStorages<'s>,
        ObjectAnimationStorages<'s, CharacterSequenceId>,
        ObjectComponentStorages<'s, CharacterSequenceId>,
        Read<'s, AssetStorage<Map>>,
    );
    impl<'s> System<'s> for TestSystem {
        type SystemData = TestSystemData<'s>;
        fn run(&mut self, _: Self::SystemData) {}
    }
}
