use amethyst::{
    animation::get_animation_set,
    assets::Asset,
    core::{nalgebra::Vector3, transform::Transform},
    ecs::prelude::*,
    renderer::{Flipped, SpriteRender, Transparent},
};
use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
use game_model::loaded::SlugAndHandle;
use log::debug;
use object_model::{
    config::object::SequenceId,
    entity::{Mirrored, Position, SequenceStatus, Velocity},
    loaded::{AnimatedComponentAnimation, AnimatedComponentDefault, GameObject},
};

use crate::{
    AnimationRunner, ObjectAnimationStorages, ObjectComponentStorages, ObjectSpawningResources,
};

/// Augments an entity with game object.
#[derive(Debug)]
pub struct ObjectEntityAugmenter;

impl ObjectEntityAugmenter {
    /// Augments an entity with `Object` components.
    ///
    /// # Parameters
    ///
    /// * `object_spawning_resources`: Resources to construct the object with.
    /// * `object_component_storages`: Common object `Component` storages.
    /// * `position`: Position of the entity in game.
    /// * `velocity`: Velocity of the entity in game.
    /// * `slug_and_handle`: Slug and handle of the object to spawn.
    pub fn augment<'s, ObTy, SeqId>(
        entity: Entity,
        ObjectSpawningResources {
            ref mut object_handles,
            object_assets,
            ref mut ob_ty_handles,
            ob_ty_assets,
        }: &mut ObjectSpawningResources<'s, ObTy, SeqId>,
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
        }: &mut ObjectComponentStorages<'s, SeqId>,
        ObjectAnimationStorages {
            ref mut sprite_render_acses,
            ref mut body_acses,
            ref mut interaction_acses,
        }: &mut ObjectAnimationStorages<'s, SeqId>,
        position: Position<f32>,
        velocity: Velocity<f32>,
        SlugAndHandle {
            ref slug,
            handle: ref ob_ty_handle,
        }: &SlugAndHandle<ObTy>,
    ) -> Entity
    where
        ObTy: Asset + GameObject<SeqId>,
        SeqId: SequenceId + 'static,
    {
        debug!("Augmenting `{}`", slug);

        let ob_ty = ob_ty_assets
            .get(ob_ty_handle)
            .unwrap_or_else(|| panic!("Expected `{}` ob_ty to be loaded.", slug));
        let object_handle = ob_ty.object_handle();
        let object = object_assets
            .get(object_handle)
            .unwrap_or_else(|| panic!("Expected `{}` object to be loaded.", slug));
        let sequence_end_transitions = ob_ty.sequence_end_transitions();

        let animation_defaults = &object.animation_defaults;

        let sequence_id = SeqId::default();
        let all_animations = object.animations.get(&sequence_id);
        let first_sequence_animations = all_animations
            .as_ref()
            .expect("Expected ob_ty to have at least one sequence.");

        let mut transform = Transform::default();
        transform.set_position(Vector3::new(position.x, position.y + position.z, 0.));

        flippeds
            .insert(entity, Flipped::None)
            .expect("Failed to insert flipped component.");
        transparents
            .insert(entity, Transparent)
            .expect("Failed to insert transparent component.");
        positions
            .insert(entity, position)
            .expect("Failed to insert position component.");
        velocities
            .insert(entity, velocity)
            .expect("Failed to insert velocity component.");
        transforms
            .insert(entity, transform)
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
        ob_ty_handles
            .insert(entity, ob_ty_handle.clone())
            .expect("Failed to insert ob_ty_handle component.");
        object_handles
            .insert(entity, object_handle.clone())
            .expect("Failed to insert object_handle component.");

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
            get_animation_set::<SeqId, SpriteRender>(sprite_render_acses, entity)
                .expect("Sprite animation should exist as new entity should be valid.");
        let mut body_animation_set =
            get_animation_set::<SeqId, BodyFrameActiveHandle>(body_acses, entity)
                .expect("Body animation should exist as new entity should be valid.");
        let mut interaction_animation_set =
            get_animation_set::<SeqId, InteractionFrameActiveHandle>(interaction_acses, entity)
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

        entity
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{
        animation::AnimationBundle,
        assets::AssetStorage,
        core::transform::Transform,
        ecs::prelude::*,
        renderer::{Flipped, SpriteRender, Transparent},
    };
    use amethyst_test::prelude::*;
    use application_event::{AppEvent, AppEventReader};
    use assets_test::{ASSETS_CHAR_BAT_SLUG, ASSETS_PATH};
    use collision_loading::CollisionLoadingBundle;
    use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
    use game_model::loaded::SlugAndHandle;
    use loading::LoadingState;
    use map_loading::MapLoadingBundle;
    use map_model::loaded::Map;
    use object_loading::ObjectLoadingBundle;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{Mirrored, Position, SequenceStatus, Velocity},
        loaded::{Character, CharacterHandle, ObjectHandle},
    };
    use typename::TypeName as TypeNameTrait;
    use typename_derive::TypeName;

    use super::ObjectEntityAugmenter;
    use crate::{
        CharacterComponentStorages, ObjectAnimationStorages, ObjectComponentStorages,
        ObjectSpawningResources,
    };

    #[test]
    fn spawn_for_player_creates_entity_with_object_components() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        let setup = |world: &mut World| {
            let position = Position::new(100., -10., -20.);
            let velocity = Velocity::default();

            let entity = world.create_entity().build();
            {
                let slug_and_handle =
                    SlugAndHandle::<Character>::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                let mut object_spawning_resources = ObjectSpawningResources::fetch(&world.res);
                let mut object_component_storages = ObjectComponentStorages::fetch(&world.res);
                let mut object_animation_storages = ObjectAnimationStorages::fetch(&world.res);
                ObjectEntityAugmenter::augment(
                    entity,
                    &mut object_spawning_resources,
                    &mut object_component_storages,
                    &mut object_animation_storages,
                    position,
                    velocity,
                    &slug_and_handle,
                );
            }

            world.add_resource(EffectReturn(entity));
        };

        let assertion = |world: &mut World| {
            let entity = world.read_resource::<EffectReturn<Entity>>().0;
            assert!(world.read_storage::<CharacterHandle>().contains(entity));
            assert!(world
                .read_storage::<ObjectHandle<CharacterSequenceId>>()
                .contains(entity));
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
            AmethystApplication::render_base(
                "spawn_for_player_creates_entity_with_object_components",
                false
            )
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
            .with_bundle(ObjectLoadingBundle::new())
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
        ObjectSpawningResources<'s, Character, CharacterSequenceId>,
        Read<'s, AssetStorage<Map>>,
    );
    impl<'s> System<'s> for TestSystem {
        type SystemData = TestSystemData<'s>;
        fn run(&mut self, _: Self::SystemData) {}
    }
}
