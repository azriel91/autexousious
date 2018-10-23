use amethyst::{
    animation::{get_animation_set, AnimationControlSet},
    assets::AssetStorage,
    core::{cgmath::Vector3, transform::Transform},
    ecs::{prelude::*, world::EntitiesRes},
    renderer::{SpriteRender, Transparent},
};
use game_input::{ControllerInput, InputControlled};
use game_model::loaded::SlugAndHandle;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, Kinematics},
    loaded::{AnimatedComponent, Character, CharacterHandle},
};

use AnimationRunner;
use CharacterComponentStorages;
use ObjectAnimationStorages;
use ObjectComponentStorages;
use ObjectSpawningResources;

/// Spawns character entities into the world.
#[derive(Debug)]
pub struct CharacterEntitySpawner;

impl CharacterEntitySpawner {
    /// Spawns a player controlled character entity.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to spawn the character into.
    /// * `kinematics`: Kinematics of the entity in game.
    /// * `slug_and_handle`: Slug of the character to spawn.
    /// * `input_controlled`: `Component` that links the character entity to the controller.
    pub fn spawn_world(
        world: &mut World,
        kinematics: Kinematics<f32>,
        slug_and_handle: &SlugAndHandle<Character>,
        input_controlled: InputControlled,
    ) -> Entity {
        let entities = &*world.read_resource::<EntitiesRes>();
        let loaded_characters = &*world.read_resource::<AssetStorage<Character>>();
        Self::spawn_system(
            &(entities, loaded_characters),
            &mut (
                world.write_storage::<InputControlled>(),
                world.write_storage::<ControllerInput>(),
                world.write_storage::<CharacterHandle>(),
                world.write_storage::<CharacterStatus>(),
            ), // kcov-ignore
            &mut (
                world.write_storage::<SpriteRender>(),
                world.write_storage::<Transparent>(),
                world.write_storage::<Kinematics<f32>>(),
                world.write_storage::<Transform>(),
            ), // kcov-ignore
            &mut (world.write_storage::<AnimationControlSet<CharacterSequenceId, SpriteRender>>(),),
            kinematics,
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
    /// * `kinematics`: Kinematics of the entity in game.
    /// * `slug_and_handle`: Slug and handle of the character to spawn.
    /// * `input_controlled`: `Component` that links the character entity to the controller.
    pub fn spawn_system<'res, 's>(
        (entities, loaded_characters): &ObjectSpawningResources<'res, Character>,
        (
            ref mut input_controlled_storage,
            ref mut controller_input_storage,
            ref mut character_handle_storage,
            ref mut character_status_storage,
        ): &mut CharacterComponentStorages<'s>,
        (
            ref mut sprite_render_storage,
            ref mut transparent_storage,
            ref mut kinematics_storage,
            ref mut transform_storage,
        ): &mut ObjectComponentStorages<'s>,
        (ref mut sprite_acs,): &mut ObjectAnimationStorages<'s, CharacterSequenceId>,
        kinematics: Kinematics<f32>,
        slug_and_handle: &SlugAndHandle<Character>,
        input_controlled: InputControlled,
    ) -> Entity {
        let character_status = CharacterStatus::default();
        let first_sequence_id = character_status.object_status.sequence_id;

        let (character_handle, sprite_render, animations) = {
            let SlugAndHandle {
                ref slug,
                ref handle,
            } = slug_and_handle;
            debug!("Spawning `{}`", slug);

            let character = loaded_characters
                .get(handle)
                .unwrap_or_else(|| panic!("Expected `{}` character to be loaded.", slug));
            let object = &character.object;

            let sprite_render = SpriteRender {
                sprite_sheet: object.default_sprite_sheet.clone(),
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            };

            let animations = object
                .animations
                .get(&first_sequence_id)
                .expect("Expected character to have at least one sequence.")
                .clone();

            (handle.clone(), sprite_render, animations)
        }; // kcov-ignore

        let position = &kinematics.position;
        let mut transform = Transform::default();
        transform.translation = Vector3::new(position.x, position.y + position.z, 0.);

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
            .insert(entity, character_handle)
            .expect("Failed to insert character_handle component.");
        // Character and object status attributes.
        character_status_storage
            .insert(entity, character_status)
            .expect("Failed to insert character_status component.");
        // The starting pose
        sprite_render_storage
            .insert(entity, sprite_render)
            .expect("Failed to insert sprite_render component.");
        // Enable transparency for visibility sorting
        transparent_storage
            .insert(entity, Transparent)
            .expect("Failed to insert transparent component.");
        // Kinematics of the entity in game.
        kinematics_storage
            .insert(entity, kinematics)
            .expect("Failed to insert kinematics component.");
        // Render location of the entity on screen.
        transform_storage
            .insert(entity, transform)
            .expect("Failed to insert transform component.");

        // We also need to trigger the animation, not just attach it to the entity
        let mut sprite_animation_set =
            get_animation_set::<CharacterSequenceId, SpriteRender>(sprite_acs, entity)
                .expect("Animation should exist as new entity should be valid.");

        animations
            .iter()
            .for_each(|animated_component| match animated_component {
                AnimatedComponent::SpriteRender(ref handle) => {
                    AnimationRunner::start(&mut sprite_animation_set, handle, first_sequence_id);
                }
            });

        entity
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{
        animation::AnimationControlSet,
        assets::AssetStorage,
        core::transform::Transform,
        ecs::prelude::*,
        renderer::{SpriteRender, Transparent},
    };
    use amethyst_test_support::prelude::*;
    use application_event::{AppEvent, AppEventReader};
    use assets_test::{ASSETS_CHAR_BAT_SLUG, ASSETS_PATH};
    use game_input::{ControllerInput, InputControlled};
    use game_model::loaded::SlugAndHandle;
    use loading::LoadingState;
    use map_loading::MapLoadingBundle;
    use map_model::loaded::Map;
    use object_loading::ObjectLoadingBundle;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{CharacterStatus, Kinematics, Position, Velocity},
        loaded::CharacterHandle,
    };
    use typename::TypeName;

    use super::CharacterEntitySpawner;

    #[test]
    fn spawn_for_player_creates_entity_with_object_components() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        let assertion = |world: &mut World| {
            let position = Position::new(100., -10., -20.);
            let kinematics = Kinematics::new(position, Velocity::default());
            let controller_id = 0;
            let input_controlled = InputControlled::new(controller_id);

            let slug_and_handle = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
            let entity = CharacterEntitySpawner::spawn_world(
                world,
                kinematics,
                &slug_and_handle,
                input_controlled,
            );

            assert!(world.read_storage::<InputControlled>().contains(entity));
            assert!(world.read_storage::<CharacterHandle>().contains(entity));
            assert!(world.read_storage::<CharacterStatus>().contains(entity));
            assert!(world.read_storage::<ControllerInput>().contains(entity));
            assert!(world.read_storage::<SpriteRender>().contains(entity));
            assert!(world.read_storage::<Transparent>().contains(entity));
            assert!(world.read_storage::<Kinematics<f32>>().contains(entity));
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
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(ObjectLoadingBundle::new())
            .with_system(TestSystem, TestSystem::type_name(), &[])
            .with_state(|| LoadingState::new(ASSETS_PATH.clone(), EmptyState))
            .with_assertion(assertion)
            .run()
            .is_ok()
        );
    }

    // Sets up storages for the various `Component`.
    #[derive(Debug, TypeName)]
    struct TestSystem;
    type TestSystemData<'s> = (
        Read<'s, AssetStorage<Map>>,
        ReadStorage<'s, InputControlled>,
        ReadStorage<'s, CharacterHandle>,
        ReadStorage<'s, CharacterStatus>,
        ReadStorage<'s, ControllerInput>,
        ReadStorage<'s, Transparent>,
        ReadStorage<'s, Kinematics<f32>>,
        ReadStorage<'s, AnimationControlSet<CharacterSequenceId, SpriteRender>>,
    );
    impl<'s> System<'s> for TestSystem {
        type SystemData = TestSystemData<'s>;
        fn run(&mut self, _: Self::SystemData) {}
    }
}
