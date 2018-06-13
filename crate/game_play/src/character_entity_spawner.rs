use amethyst::{
    animation::get_animation_set,
    assets::AssetStorage,
    core::transform::{GlobalTransform, Transform},
    ecs::prelude::*,
    renderer::Material,
};
use character_selection::CharacterEntityControl;
use object_model::{
    config::object::character::SequenceId,
    entity::ObjectStatus,
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
    /// * `transform`: Position of the entity in the world.
    /// * `character_index`: Index of the character to spawn.
    /// * `character_entity_control`: `Component` that links the character entity to the controller.
    pub(crate) fn spawn_for_player(
        world: &mut World,
        transform: Transform,
        character_index: usize,
        character_entity_control: CharacterEntityControl,
    ) -> Entity {
        let first_sequence_id = SequenceId::default();

        let (character_handle, material, mesh, animation_handle) = {
            let loaded_characters = world.read_resource::<Vec<CharacterHandle>>();

            let character_handle = loaded_characters.get(character_index).unwrap_or_else(|| {
                let error_msg = format!(
                    "Attempted to spawn character at index: `{}` for `{:?}`, \
                     but index is out of bounds.",
                    character_index, &character_entity_control
                );
                panic!(error_msg)
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
                    .get(&first_sequence_id)
                    .expect("Expected character to have at least one sequence.")
                    .clone(),
            ) // kcov-ignore
        };

        let entity = world
            .create_entity()
            // Controller of this entity
            .with(character_entity_control)
            // Loaded `Character` for this entity.
            .with(character_handle)
            // The default `Material`, whose textures will be swapped based on the animation.
            .with(material)
            // Shift sprite to some part of the window
            .with(mesh)
            // Location of the entity
            .with(transform)
            // This defines the coordinates in the world, where the sprites should be drawn relative
            // to the entity
            .with(GlobalTransform::default())
            // Set the default sequence for the object
            .with(ObjectStatus::new(first_sequence_id))
            .build();

        // We also need to trigger the animation, not just attach it to the entity
        let mut animation_control_set_storage = world.write_storage();
        let mut animation_set =
            get_animation_set::<SequenceId, Material>(&mut animation_control_set_storage, entity);

        AnimationRunner::start(&mut animation_set, &animation_handle, &first_sequence_id);

        entity
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        animation::AnimationBundle,
        core::{
            cgmath::Vector3,
            transform::{GlobalTransform, Transform, TransformBundle},
        },
        input::InputBundle,
        prelude::*,
        renderer::{
            ColorMask, DisplayConfig, DrawFlat, Material, MeshHandle, Pipeline, PosTex,
            RenderBundle, Stage, ALPHA,
        },
        ui::UiBundle,
        Result,
    };
    use application::resource::{
        self,
        dir::{self, assets_dir},
        load_in,
    };
    use character_selection::CharacterEntityControl;
    use loading;
    use object_loading::ObjectLoadingBundle;
    use object_model::{
        config::object::character::SequenceId, entity::ObjectStatus, loaded::CharacterHandle,
    };

    use super::CharacterEntitySpawner;
    use GamePlayBundle;

    #[test]
    fn spawn_for_player_creates_entity_with_object_components() {
        let assertion_fn = |world: &mut World| {
            let mut transform = Transform::default();
            transform.translation = Vector3::new(100., -10., -20.);
            let character_index = 0;
            let controller_id = 0;
            let character_entity_control = CharacterEntityControl::new(controller_id);

            let entity = CharacterEntitySpawner::spawn_for_player(
                world,
                transform,
                character_index,
                character_entity_control,
            );

            assert!(
                world
                    .read_storage::<CharacterEntityControl>()
                    .contains(entity)
            );
            assert!(world.read_storage::<CharacterHandle>().contains(entity));
            assert!(world.read_storage::<Material>().contains(entity));
            assert!(world.read_storage::<MeshHandle>().contains(entity));
            assert!(world.read_storage::<Transform>().contains(entity));
            assert!(world.read_storage::<GlobalTransform>().contains(entity));
            assert!(
                world
                    .read_storage::<ObjectStatus<SequenceId>>()
                    .contains(entity)
            );
        };

        assert!(run(Box::new(assertion_fn)).is_ok())
    }

    fn run<F>(assertion_fn: Box<F>) -> Result<()>
    where
        F: 'static + Fn(&mut World),
    {
        let assets_dir = assets_dir(Some(development_base_dirs!()))?;
        let test_state = TestState { assertion_fn };
        let loading_state = loading::State::new(assets_dir.clone(), Box::new(test_state));

        Application::new(assets_dir, loading_state, setup_game_data()?)?.run();

        Ok(())
    }

    fn setup_game_data<'a, 'b>() -> Result<GameDataBuilder<'a, 'b>> {
        let pipeline = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0., 0., 0., 0.], 1.)
                .with_pass(DrawFlat::<PosTex>::new().with_transparency(
                    ColorMask::all(),
                    ALPHA,
                    None,
                )),
        );

        GameDataBuilder::default()
            // Provides sprite animation
            .with_bundle(AnimationBundle::<u32, Material>::new(
                "animation_control_system",
                "sampler_interpolation_system",
            ))?
            // Handles transformations of textures
            .with_bundle(
                TransformBundle::new()
                    .with_dep(&["animation_control_system", "sampler_interpolation_system"]),
            )?
            .with_bundle(RenderBundle::new(pipeline, Some(display_config()?)))?
            .with_bundle(InputBundle::<String, String>::new())?
            .with_bundle(UiBundle::<String, String>::new())?
            .with_bundle(ObjectLoadingBundle::new())?
            .with_bundle(GamePlayBundle) // Needed for `CharacterEntityControl`
    } // kcov-ignore

    fn display_config() -> Result<DisplayConfig> {
        Ok(load_in::<DisplayConfig, _>(
            dir::RESOURCES,
            "display_config.ron",
            &resource::Format::Ron,
            Some(development_base_dirs!()),
        )?)
    }

    #[derive(Debug)]
    struct TestState<F: Fn(&mut World)> {
        assertion_fn: Box<F>,
    }
    impl<'a, 'b, F: Fn(&mut World)> State<GameData<'a, 'b>> for TestState<F> {
        fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
            data.data.update(&data.world);
            Trans::None
        }

        fn fixed_update(&mut self, mut data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
            // This needs to be in `fixed_update`:
            //
            // > Loading some assets requires a renderer tick @azriel91 because only the rendering
            // > thread can load them
            // >
            // > - Xaeroxe
            (self.assertion_fn)(&mut data.world);

            Trans::Quit
        }
    }
}
