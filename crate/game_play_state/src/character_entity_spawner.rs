use amethyst::{
    animation::get_animation_set,
    assets::AssetStorage,
    core::{
        cgmath::Vector3,
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::*,
    renderer::Material,
};
use character_selection::CharacterEntityControl;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, Kinematics},
    loaded::{Character, CharacterHandle},
};

use AnimationRunner;

/// Spawns character entities into the world.
#[derive(Debug)]
pub(crate) struct CharacterEntitySpawner;

impl CharacterEntitySpawner {
    /// Spawns a player controlled character entity.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to spawn the character into.
    /// * `kinematics`: Kinematics of the entity in game.
    /// * `character_index`: Index of the character to spawn.
    /// * `character_entity_control`: `Component` that links the character entity to the controller.
    pub(crate) fn spawn_for_player(
        world: &mut World,
        kinematics: Kinematics<f32>,
        character_index: usize,
        character_entity_control: CharacterEntityControl,
    ) -> Entity {
        let character_status = CharacterStatus::default();
        let first_sequence_id = &character_status.object_status.sequence_id;

        let (character_handle, material, mesh, animation_handle) = {
            let loaded_characters = world.read_resource::<Vec<CharacterHandle>>();

            let character_handle = loaded_characters.get(character_index).unwrap_or_else(|| {
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

            let store = world.read_resource::<AssetStorage<Character>>();
            let character = store
                .get(character_handle)
                .expect("Expected character to be loaded.");

            (
                character_handle.clone(),
                character.object.default_material.clone(),
                character.object.mesh.clone(),
                character
                    .object
                    .animations
                    .get(first_sequence_id)
                    .expect("Expected character to have at least one sequence.")
                    .clone(),
            ) // kcov-ignore
        };

        let position = &kinematics.position;
        let mut transform = Transform::default();
        transform.translation = Vector3::new(position.x, position.y + position.z, 0.);

        let entity = world
            .create_entity()
            // Controller of this entity
            .with(character_entity_control)
            // Loaded `Character` for this entity.
            .with(character_handle)
            // Character and object status attributes.
            .with(character_status)
            // The default `Material`, whose textures will be swapped based on the animation.
            .with(material)
            // Coordinates to map the sprite texture to screen. This is the non-mirrored mesh.
            .with(mesh)
            // Kinematics of the entity in game.
            .with(kinematics)
            // Render location of the entity on screen.
            .with(transform)
            // This defines the coordinates in the world, where the sprites should be drawn relative
            // to the entity
            .with(GlobalTransform::default())
            .build();

        // We also need to trigger the animation, not just attach it to the entity
        let mut animation_control_set_storage = world.write_storage();
        let mut animation_set = get_animation_set::<CharacterSequenceId, Material>(
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
        renderer::{Material, MeshHandle},
    };
    use amethyst_test_support::prelude::*;
    use character_selection::CharacterEntityControl;
    use loading;
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
            assert!(world.read_storage::<Material>().contains(entity));
            assert!(world.read_storage::<MeshHandle>().contains(entity));
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
            ))
                .with_assertion(assertion)
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
        ReadStorage<'s, AnimationControlSet<CharacterSequenceId, Material>>,
    );
    impl<'s> System<'s> for TestSystem {
        type SystemData = TestSystemData<'s>;
        fn run(&mut self, _: Self::SystemData) {}
    }
}
