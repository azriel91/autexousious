use amethyst::{
    animation::get_animation_set,
    assets::AssetStorage,
    core::{nalgebra::Vector3, transform::Transform},
    ecs::{prelude::*, world::EntitiesRes},
    renderer::{SpriteRender, Transparent},
};
use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
use game_input::{ControllerInput, InputControlled};
use game_model::loaded::SlugAndHandle;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{Grounding, HealthPoints, Mirrored, Position, RunCounter, SequenceStatus, Velocity},
    loaded::{
        AnimatedComponentAnimation, AnimatedComponentDefault, Character, CharacterHandle, Object,
        ObjectHandle, SequenceEndTransitions,
    },
};

use crate::AnimationRunner;
use crate::BodyAcs;
use crate::CharacterComponentStorages;
use crate::InteractionAcs;
use crate::ObjectAnimationStorages;
use crate::ObjectComponentStorages;
use crate::ObjectSpawningResources;
use crate::SpriteRenderAcs;

/// Spawns character entities into the world.
#[derive(Debug)]
pub struct CharacterEntitySpawner;

impl CharacterEntitySpawner {
    /// Spawns a player controlled character entity.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to spawn the character into.
    /// * `position`: Position of the entity in game.
    /// * `velocity`: Velocity of the entity in game.
    /// * `slug_and_handle`: Slug of the character to spawn.
    /// * `input_controlled`: `Component` that links the character entity to the controller.
    pub fn spawn_world(
        world: &mut World,
        position: Position<f32>,
        velocity: Velocity<f32>,
        slug_and_handle: &SlugAndHandle<Character>,
        input_controlled: InputControlled,
    ) -> Entity {
        let entities = Read::from(world.read_resource::<EntitiesRes>());
        let loaded_characters = Read::from(world.read_resource::<AssetStorage<Character>>());
        let loaded_objects =
            Read::from(world.read_resource::<AssetStorage<Object<CharacterSequenceId>>>());
        Self::spawn_system(
            &(entities, loaded_characters, loaded_objects),
            &mut (
                world.write_storage::<InputControlled>(),
                world.write_storage::<ControllerInput>(),
                world.write_storage::<CharacterHandle>(),
                world.write_storage::<ObjectHandle<CharacterSequenceId>>(),
                world.write_storage::<SequenceEndTransitions<CharacterSequenceId>>(),
                world.write_storage::<HealthPoints>(),
                world.write_storage::<CharacterSequenceId>(),
                world.write_storage::<SequenceStatus>(),
                world.write_storage::<RunCounter>(),
                world.write_storage::<Mirrored>(),
                world.write_storage::<Grounding>(),
            ), // kcov-ignore
            &mut (
                world.write_storage::<SpriteRender>(),
                world.write_storage::<Transparent>(),
                world.write_storage::<Position<f32>>(),
                world.write_storage::<Velocity<f32>>(),
                world.write_storage::<Transform>(),
                world.write_storage::<BodyFrameActiveHandle>(),
                world.write_storage::<InteractionFrameActiveHandle>(),
            ), // kcov-ignore
            &mut (
                world.write_storage::<SpriteRenderAcs<CharacterSequenceId>>(),
                world.write_storage::<BodyAcs<CharacterSequenceId>>(),
                world.write_storage::<InteractionAcs<CharacterSequenceId>>(),
            ),
            position,
            velocity,
            slug_and_handle,
            input_controlled,
        )
    }

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
    pub fn spawn_system<'res, 's>(
        (entities, loaded_characters, loaded_objects): &ObjectSpawningResources<
            'res,
            Character,
            CharacterSequenceId,
        >,
        (
            ref mut input_controlled_storage,
            ref mut controller_input_storage,
            ref mut character_handle_storage,
            ref mut object_handle_storage,
            ref mut sequence_end_transitions_storage,
            ref mut health_points_storage,
            ref mut character_sequence_ids,
            ref mut sequence_status_storage,
            ref mut run_counter_storage,
            ref mut mirrored_storage,
            ref mut grounding_storage,
        ): &mut CharacterComponentStorages<'s>,
        (
            ref mut sprite_render_storage,
            ref mut transparent_storage,
            ref mut position_storage,
            ref mut velocity_storage,
            ref mut transform_storage,
            ref mut body_frame_active_handle_storage,
            ref mut interaction_frame_active_handle_storage,
        ): &mut ObjectComponentStorages<'s>,
        (ref mut sprite_acs, ref mut body_frame_acs, ref mut interaction_acs): &mut ObjectAnimationStorages<
            's,
            CharacterSequenceId,
        >,
        position: Position<f32>,
        velocity: Velocity<f32>,
        slug_and_handle: &SlugAndHandle<Character>,
        input_controlled: InputControlled,
    ) -> Entity {
        let character_sequence_id = CharacterSequenceId::default();

        let SlugAndHandle {
            ref slug,
            handle: ref character_handle,
        } = slug_and_handle;

        debug!("Spawning `{}`", slug);

        let character = loaded_characters
            .get(character_handle)
            .unwrap_or_else(|| panic!("Expected `{}` character to be loaded.", slug));
        let object_handle = &character.object_handle;
        let object = loaded_objects
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

        let entity = entities.create();

        // Controller of this entity
        input_controlled_storage
            .insert(entity, input_controlled)
            .expect("Failed to insert input_controlled component.");
        // Controller of this entity
        controller_input_storage
            .insert(entity, ControllerInput::default())
            .expect("Failed to insert controller_input component.");
        // Loaded `Character` for this entity.
        character_handle_storage
            .insert(entity, character_handle.clone())
            .expect("Failed to insert character_handle component.");
        // Loaded animations.
        object_handle_storage
            .insert(entity, object_handle.clone())
            .expect("Failed to insert object_handle component.");
        // Loaded animations.
        sequence_end_transitions_storage
            .insert(entity, sequence_end_transitions.clone())
            .expect("Failed to insert sequence_end_transitions component.");
        // Health points.
        health_points_storage
            .insert(entity, HealthPoints::default())
            .expect("Failed to insert health_points component.");
        // Object status attributes.
        character_sequence_ids
            .insert(entity, character_sequence_id)
            .expect("Failed to insert character_sequence_id component.");
        // Sequence status attributes.
        sequence_status_storage
            .insert(entity, SequenceStatus::default())
            .expect("Failed to insert sequence_status component.");
        // Run counter.
        run_counter_storage
            .insert(entity, RunCounter::default())
            .expect("Failed to insert run_counter component.");
        // Mirrored.
        mirrored_storage
            .insert(entity, Mirrored::default())
            .expect("Failed to insert mirrored component.");
        // Grounding.
        grounding_storage
            .insert(entity, Grounding::default())
            .expect("Failed to insert grounding component.");
        // Enable transparency for visibility sorting
        transparent_storage
            .insert(entity, Transparent)
            .expect("Failed to insert transparent component.");
        // Position of the entity in game.
        position_storage
            .insert(entity, position)
            .expect("Failed to insert position component.");
        // Velocity of the entity in game.
        velocity_storage
            .insert(entity, velocity)
            .expect("Failed to insert velocity component.");
        // Render location of the entity on screen.
        transform_storage
            .insert(entity, transform)
            .expect("Failed to insert transform component.");

        animation_defaults
            .iter()
            .for_each(|animation_default| match animation_default {
                AnimatedComponentDefault::SpriteRender(ref sprite_render) => {
                    // The starting pose
                    sprite_render_storage
                        .insert(entity, sprite_render.clone())
                        .expect("Failed to insert `SpriteRender` component.");
                }
                AnimatedComponentDefault::BodyFrame(ref active_handle) => {
                    // Default body active handle
                    body_frame_active_handle_storage
                        .insert(entity, active_handle.clone())
                        .expect("Failed to insert `BodyFrameActiveHandle` component.");
                }
                AnimatedComponentDefault::InteractionFrame(ref active_handle) => {
                    // Default interaction active handle
                    interaction_frame_active_handle_storage
                        .insert(entity, active_handle.clone())
                        .expect("Failed to insert `InteractionFrameActiveHandle` component.");
                }
            });

        // We also need to trigger the animation, not just attach it to the entity
        let mut sprite_animation_set =
            get_animation_set::<CharacterSequenceId, SpriteRender>(sprite_acs, entity)
                .expect("Sprite animation should exist as new entity should be valid.");
        let mut body_animation_set =
            get_animation_set::<CharacterSequenceId, BodyFrameActiveHandle>(body_frame_acs, entity)
                .expect("Body animation should exist as new entity should be valid.");
        let mut interaction_animation_set = get_animation_set::<
            CharacterSequenceId,
            InteractionFrameActiveHandle,
        >(interaction_acs, entity)
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
        renderer::{SpriteRender, Transparent},
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
    use typename::TypeName;

    use super::CharacterEntitySpawner;
    use crate::CharacterComponentStorages;
    use crate::ObjectAnimationStorages;
    use crate::ObjectComponentStorages;
    use crate::ObjectSpawningResources;

    #[test]
    fn spawn_for_player_creates_entity_with_object_components() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        let assertion = |world: &mut World| {
            let position = Position::new(100., -10., -20.);
            let velocity = Velocity::default();
            let controller_id = 0;
            let input_controlled = InputControlled::new(controller_id);

            let slug_and_handle = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
            let entity = CharacterEntitySpawner::spawn_world(
                world,
                position,
                velocity,
                &slug_and_handle,
                input_controlled,
            );

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
        ObjectComponentStorages<'s>,
        ObjectSpawningResources<'s, Character, CharacterSequenceId>,
        Read<'s, AssetStorage<Map>>,
    );
    impl<'s> System<'s> for TestSystem {
        type SystemData = TestSystemData<'s>;
        fn run(&mut self, _: Self::SystemData) {}
    }
}
