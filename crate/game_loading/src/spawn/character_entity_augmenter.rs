use amethyst::{
    animation::get_animation_set,
    core::{nalgebra::Vector3, transform::Transform},
    ecs::prelude::*,
    renderer::{Flipped, SpriteRender, Transparent},
};
use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
use game_input::{ControllerInput, InputControlled};
use game_model::loaded::SlugAndHandle;
use log::debug;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{Grounding, HealthPoints, Mirrored, Position, RunCounter, SequenceStatus, Velocity},
    loaded::{AnimatedComponentAnimation, AnimatedComponentDefault, Character},
};

use crate::{
    AnimationRunner, CharacterComponentStorages, ObjectAnimationStorages, ObjectComponentStorages,
    ObjectSpawningResources,
};

/// Spawns character entities into the world.
#[derive(Debug)]
pub struct CharacterEntityAugmenter;

impl CharacterEntityAugmenter {
    /// Spawns a player controlled character entity.
    ///
    /// # Parameters
    ///
    /// * `object_spawning_resources`: Resources to construct the character with.
    /// * `character_component_storages`: Character specific `Component` storages.
    /// * `object_component_storages`: Common object `Component` storages.
    /// * `position`: Position of the entity in game.
    /// * `velocity`: Velocity of the entity in game.
    /// * `slug_and_handle`: Slug and handle of the character to spawn.
    /// * `input_controlled`: `Component` that links the character entity to the controller.
    pub fn augment<'res, 's>(
        entity: Entity,
        ObjectSpawningResources {
            ref mut object_handles,
            object_assets,
            ref mut ob_ty_handles,
            ob_ty_assets,
        }: &mut ObjectSpawningResources<'res, Character, CharacterSequenceId>,
        CharacterComponentStorages {
            ref mut input_controlleds,
            ref mut controller_inputs,
            ref mut health_pointses,
            ref mut run_counters,
            ref mut groundings,
        }: &mut CharacterComponentStorages<'s>,
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
        }: &mut ObjectComponentStorages<'s, CharacterSequenceId>,
        ObjectAnimationStorages {
            ref mut sprite_render_acses,
            ref mut body_acses,
            ref mut interaction_acses,
        }: &mut ObjectAnimationStorages<'s, CharacterSequenceId>,
        position: Position<f32>,
        velocity: Velocity<f32>,
        slug_and_handle: &SlugAndHandle<Character>,
        input_controlled: InputControlled,
    ) {
        let character_sequence_id = CharacterSequenceId::default();

        let SlugAndHandle {
            ref slug,
            handle: ref character_handle,
        } = slug_and_handle;

        debug!("Spawning `{}`", slug);

        let character = ob_ty_assets
            .get(character_handle)
            .unwrap_or_else(|| panic!("Expected `{}` character to be loaded.", slug));
        let object_handle = &character.object_handle;
        let object = object_assets
            .get(object_handle)
            .unwrap_or_else(|| panic!("Expected `{}` object to be loaded.", slug));
        let sequence_end_transitions = &character.sequence_end_transitions;

        let animation_defaults = &object.animation_defaults;

        let all_animations = object.animations.get(&character_sequence_id);
        let first_sequence_animations = all_animations
            .as_ref()
            .expect("Expected character to have at least one sequence.");

        let mut transform = Transform::default();
        transform.set_position(Vector3::new(position.x, position.y + position.z, 0.));

        // Controller of this entity
        input_controlleds
            .insert(entity, input_controlled)
            .expect("Failed to insert input_controlled component.");
        // Controller of this entity
        controller_inputs
            .insert(entity, ControllerInput::default())
            .expect("Failed to insert controller_input component.");
        // Loaded `Character` for this entity.
        ob_ty_handles
            .insert(entity, character_handle.clone())
            .expect("Failed to insert character_handle component.");
        // Loaded animations.
        object_handles
            .insert(entity, object_handle.clone())
            .expect("Failed to insert object_handle component.");
        // Loaded animations.
        sequence_end_transitionses
            .insert(entity, sequence_end_transitions.clone())
            .expect("Failed to insert sequence_end_transitions component.");
        // Health points.
        health_pointses
            .insert(entity, HealthPoints::default())
            .expect("Failed to insert health_points component.");
        // Object status attributes.
        sequence_ids
            .insert(entity, character_sequence_id)
            .expect("Failed to insert character_sequence_id component.");
        // Sequence status attributes.
        sequence_statuses
            .insert(entity, SequenceStatus::default())
            .expect("Failed to insert sequence_status component.");
        // Run counter.
        run_counters
            .insert(entity, RunCounter::default())
            .expect("Failed to insert run_counter component.");
        // Mirrored.
        mirroreds
            .insert(entity, Mirrored::default())
            .expect("Failed to insert mirrored component.");
        // Grounding.
        groundings
            .insert(entity, Grounding::default())
            .expect("Failed to insert grounding component.");
        // Whether the sprite should be flipped
        flippeds
            .insert(entity, Flipped::None)
            .expect("Failed to insert flipped component.");
        // Enable transparency for visibility sorting
        transparents
            .insert(entity, Transparent)
            .expect("Failed to insert transparent component.");
        // Position of the entity in game.
        positions
            .insert(entity, position)
            .expect("Failed to insert position component.");
        // Velocity of the entity in game.
        velocities
            .insert(entity, velocity)
            .expect("Failed to insert velocity component.");
        // Render location of the entity on screen.
        transforms
            .insert(entity, transform)
            .expect("Failed to insert transform component.");

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
            get_animation_set::<CharacterSequenceId, SpriteRender>(sprite_render_acses, entity)
                .expect("Sprite animation should exist as new entity should be valid.");
        let mut body_animation_set =
            get_animation_set::<CharacterSequenceId, BodyFrameActiveHandle>(body_acses, entity)
                .expect("Body animation should exist as new entity should be valid.");
        let mut interaction_animation_set = get_animation_set::<
            CharacterSequenceId,
            InteractionFrameActiveHandle,
        >(interaction_acses, entity)
        .expect("Interaction animation should exist as new entity should be valid.");

        first_sequence_animations
            .iter()
            .for_each(|animated_component| match animated_component {
                AnimatedComponentAnimation::SpriteRender(ref handle) => {
                    AnimationRunner::start(
                        character_sequence_id,
                        &mut sprite_animation_set,
                        handle,
                    );
                }
                AnimatedComponentAnimation::BodyFrame(ref handle) => {
                    AnimationRunner::start(character_sequence_id, &mut body_animation_set, handle);
                }
                AnimatedComponentAnimation::InteractionFrame(ref handle) => {
                    AnimationRunner::start(
                        character_sequence_id,
                        &mut interaction_animation_set,
                        handle,
                    );
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
        ecs::prelude::*,
        renderer::{Flipped, SpriteRender, Transparent},
    };
    use amethyst_test::prelude::*;
    use application_event::{AppEvent, AppEventReader};
    use assets_test::{ASSETS_CHAR_BAT_SLUG, ASSETS_PATH};
    use collision_loading::CollisionLoadingBundle;
    use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
    use game_input::{ControllerInput, InputControlled};
    use game_model::loaded::SlugAndHandle;
    use loading::LoadingState;
    use map_loading::MapLoadingBundle;
    use map_model::loaded::Map;
    use object_loading::ObjectLoadingBundle;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{Grounding, HealthPoints, Mirrored, Position, SequenceStatus, Velocity},
        loaded::{Character, CharacterHandle, ObjectHandle},
    };
    use typename::TypeName as TypeNameTrait;
    use typename_derive::TypeName;

    use super::CharacterEntityAugmenter;
    use crate::{
        CharacterComponentStorages, ObjectAnimationStorages, ObjectComponentStorages,
        ObjectSpawningResources,
    };

    #[test]
    fn spawn_for_player_creates_entity_with_object_components() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        let assertion = |world: &mut World| {
            let position = Position::new(100., -10., -20.);
            let velocity = Velocity::default();
            let controller_id = 0;
            let input_controlled = InputControlled::new(controller_id);

            let entity = world.create_entity().build();
            {
                let slug_and_handle =
                    SlugAndHandle::<Character>::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
                let mut object_spawning_resources = ObjectSpawningResources::fetch(&world.res);
                let mut character_component_storages =
                    CharacterComponentStorages::fetch(&world.res);
                let mut object_component_storages = ObjectComponentStorages::fetch(&world.res);
                let mut object_animation_storages = ObjectAnimationStorages::fetch(&world.res);
                CharacterEntityAugmenter::augment(
                    entity,
                    &mut object_spawning_resources,
                    &mut character_component_storages,
                    &mut object_component_storages,
                    &mut object_animation_storages,
                    position,
                    velocity,
                    &slug_and_handle,
                    input_controlled,
                );
            }

            assert!(world.read_storage::<InputControlled>().contains(entity));
            assert!(world.read_storage::<ControllerInput>().contains(entity));
            assert!(world.read_storage::<CharacterHandle>().contains(entity));
            assert!(world
                .read_storage::<ObjectHandle<CharacterSequenceId>>()
                .contains(entity));
            assert!(world.read_storage::<HealthPoints>().contains(entity));
            assert!(world.read_storage::<CharacterSequenceId>().contains(entity));
            assert!(world.read_storage::<SequenceStatus>().contains(entity));
            assert!(world.read_storage::<Mirrored>().contains(entity));
            assert!(world.read_storage::<Grounding>().contains(entity));
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
