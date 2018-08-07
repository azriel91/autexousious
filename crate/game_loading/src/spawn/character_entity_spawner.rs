use amethyst::{
    animation::{get_animation_set, AnimationControlSet},
    assets::AssetStorage,
    core::{
        cgmath::Vector3,
        transform::{GlobalTransform, Transform},
    },
    ecs::{prelude::*, world::EntitiesRes},
    renderer::SpriteRender,
};
use character_selection::CharacterEntityControl;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, Kinematics},
    loaded::{Character, CharacterHandle},
};

use AnimationRunner;
use CharacterComponentStorages;
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
    /// * `character_index`: Index of the character to spawn.
    /// * `character_entity_control`: `Component` that links the character entity to the controller.
    #[allow(unknown_lints)]
    #[allow(let_and_return)]
    pub fn spawn_for_player(
        world: &mut World,
        kinematics: Kinematics<f32>,
        character_index: usize,
        character_entity_control: CharacterEntityControl,
    ) -> Entity {
        let entities = world.read_resource::<EntitiesRes>();
        let loaded_character_handles = world.read_resource::<Vec<CharacterHandle>>();
        let loaded_characters = world.read_resource::<AssetStorage<Character>>();
        let entity = Self::spawn_system(
            (&*entities, &*loaded_character_handles, &*loaded_characters),
            (
                world.write_storage::<CharacterEntityControl>(),
                world.write_storage::<CharacterHandle>(),
                world.write_storage::<CharacterStatus>(),
            ),
            (
                world.write_storage::<SpriteRender>(),
                world.write_storage::<Kinematics<f32>>(),
                world.write_storage::<Transform>(),
                world.write_storage::<GlobalTransform>(),
                world.write_storage::<AnimationControlSet<CharacterSequenceId, SpriteRender>>(),
            ),
            kinematics,
            character_index,
            character_entity_control,
        );

        entity
    }

    /// Spawns a player controlled character entity.
    ///
    /// # Parameters
    ///
    /// * `kinematics`: Kinematics of the entity in game.
    /// * `character_index`: Index of the character to spawn.
    /// * `character_entity_control`: `Component` that links the character entity to the controller.
    pub fn spawn_system<'s>(
        (entities, loaded_character_handles, loaded_characters): ObjectSpawningResources<
            's,
            Character,
        >,
        (
            mut character_entity_control_storage,
            mut character_handle_storage,
            mut character_status_storage,
        ): CharacterComponentStorages<'s>,
        (
            mut sprite_render_storage,
            mut kinematics_storage,
            mut transform_storage,
            mut global_transform_storage,
            mut animation_control_set_storage,
        ): ObjectComponentStorages<'s, CharacterSequenceId>,
        kinematics: Kinematics<f32>,
        character_index: usize,
        character_entity_control: CharacterEntityControl,
    ) -> Entity {
        let character_status = CharacterStatus::default();
        let first_sequence_id = character_status.object_status.sequence_id;

        let (character_handle, sprite_render, animation_handle) = {
            let character_handle = loaded_character_handles
                .get(character_index)
                .unwrap_or_else(|| {
                    // kcov-ignore-start
                    let error_msg = format!(
                        "Attempted to spawn character at index: `{}` for `{:?}`, \
                         but index is out of bounds.",
                        character_index, &character_entity_control
                    );
                    panic!(error_msg)
                    // kcov-ignore-end
                });

            debug!("Retrieving character with handle: `{:?}`", character_handle);

            let character = loaded_characters
                .get(character_handle)
                .expect("Expected character to be loaded.");
            let object = &character.object;

            let sprite_render = SpriteRender {
                sprite_sheet: object.default_sprite_sheet.clone(),
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            };

            let animation_handle = object
                .animations
                .get(&first_sequence_id)
                .expect("Expected character to have at least one sequence.")
                .clone();

            (character_handle.clone(), sprite_render, animation_handle)
        };

        let position = &kinematics.position;
        let mut transform = Transform::default();
        transform.translation = Vector3::new(position.x, position.y + position.z, 0.);

        let entity = entities.create();

        // Controller of this entity
        character_entity_control_storage
            .insert(entity, character_entity_control)
            .expect("Failed to insert character_entity_control component.");
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
        // Kinematics of the entity in game.
        kinematics_storage
            .insert(entity, kinematics)
            .expect("Failed to insert kinematics component.");
        // Render location of the entity on screen.
        transform_storage
            .insert(entity, transform)
            .expect("Failed to insert transform component.");
        // This defines the coordinates in the world, where the sprites should be drawn relative
        // to the entity
        global_transform_storage
            .insert(entity, GlobalTransform::default())
            .expect("Failed to insert global_transform component.");;

        // We also need to trigger the animation, not just attach it to the entity
        let mut animation_set = get_animation_set::<CharacterSequenceId, SpriteRender>(
            &mut animation_control_set_storage,
            entity,
        ).expect("Animation should exist as new entity should be valid.");

        AnimationRunner::start(&mut animation_set, &animation_handle, first_sequence_id);

        entity
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{
        animation::AnimationControlSet,
        assets::AssetStorage,
        core::transform::{GlobalTransform, Transform},
        ecs::prelude::*,
        renderer::SpriteRender,
    };
    use amethyst_test_support::prelude::*;
    use character_selection::CharacterEntityControl;
    use loading;
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
            let character_index = 0;
            let controller_id = 0;
            let character_entity_control = CharacterEntityControl::new(controller_id);

            let entity = CharacterEntitySpawner::spawn_for_player(
                world,
                kinematics,
                character_index,
                character_entity_control,
            );

            assert!(
                world
                    .read_storage::<CharacterEntityControl>()
                    .contains(entity)
            );
            assert!(world.read_storage::<CharacterHandle>().contains(entity));
            assert!(world.read_storage::<CharacterStatus>().contains(entity));
            assert!(world.read_storage::<SpriteRender>().contains(entity));
            assert!(world.read_storage::<Kinematics<f32>>().contains(entity));
            assert!(world.read_storage::<Transform>().contains(entity));
            assert!(world.read_storage::<GlobalTransform>().contains(entity));
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base(
                "spawn_for_player_creates_entity_with_object_components",
                false
            ).with_state(|| loading::State::new(
                AmethystApplication::assets_dir().into(),
                Box::new(EmptyState),
            )).with_assertion(assertion)
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(ObjectLoadingBundle::new())
            .with_system(TestSystem, TestSystem::type_name(), &[])
            .run()
            .is_ok()
        );
    }

    // Sets up storages for the various `Component`.
    #[derive(Debug, TypeName)]
    struct TestSystem;
    type TestSystemData<'s> = (
        Read<'s, AssetStorage<Map>>,
        ReadStorage<'s, CharacterEntityControl>,
        ReadStorage<'s, CharacterHandle>,
        ReadStorage<'s, CharacterStatus>,
        ReadStorage<'s, Kinematics<f32>>,
        ReadStorage<'s, AnimationControlSet<CharacterSequenceId, SpriteRender>>,
    );
    impl<'s> System<'s> for TestSystem {
        type SystemData = TestSystemData<'s>;
        fn run(&mut self, _: Self::SystemData) {}
    }
}
