use std::env;
use std::path::Path;

use amethyst::{
    animation::AnimationBundle, core::transform::TransformBundle, prelude::*,
    renderer::SpriteRender,
};
use amethyst_test_support::{prelude::*, EmptyState};
use application::resource::dir::ASSETS;
use character_selection::{
    CharacterSelectionBundle, CharacterSelections, CharacterSelectionsState,
};
use game_input::{PlayerActionControl, PlayerAxisControl};
use game_loading::GameLoadingState;
use loading::LoadingState;
use map_loading::MapLoadingBundle;
use map_model::loaded::MapHandle;
use map_selection::{MapSelection, MapSelectionStatus};
use object_loading::ObjectLoadingBundle;
use object_model::config::object::CharacterSequenceId;

/// Baselines for building Amethyst applications with Autexousious types.
#[derive(Debug)]
pub struct AutexousiousApplication;

impl AutexousiousApplication {
    /// Returns an application with the Transform, Input, and UI bundles.
    ///
    /// This also adds a `ScreenDimensions` resource to the `World` so that UI calculations can be
    /// done.
    ///
    /// This has the same effect as calling `AmethystApplication::base::<PlayerAxisControl,
    /// PlayerActionControl>()`.
    pub fn ui_base() -> AmethystApplication<GameData<'static, 'static>, ()> {
        AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
    }

    /// Returns an application with the Animation, Transform, and Render bundles.
    ///
    /// The difference between this and `AmethystApplication::render_base()` is the type parameters
    /// to the Input and UI bundles are the `PlayerAxisControl` and `PlayerActionControl`, and the
    /// Animation bundle uses the object type sequence IDs for animation control sets.
    ///
    /// # Parameters
    ///
    /// * `test_name`: Name of the test, used to populate the window title.
    /// * `visibility`: Whether the window should be visible.
    pub fn render_base<'name, N>(
        test_name: N,
        visibility: bool,
    ) -> AmethystApplication<GameData<'static, 'static>, ()>
    where
        N: Into<&'name str>,
    {
        // Unfortunately we cannot re-use `AmethystApplication::render_base` because we need to
        // specify the `TransformBundle`'s dependencies.
        AmethystApplication::blank()
            .with_bundle(AnimationBundle::<CharacterSequenceId, SpriteRender>::new(
                "character_animation_control_system",
                "character_sampler_interpolation_system",
            )).with_bundle(TransformBundle::new().with_dep(&[
                "character_animation_control_system",
                "character_sampler_interpolation_system",
            ])).with_render_bundle(test_name, visibility)
    }

    /// Returns an application with Render, Input, and UI bundles loaded.
    ///
    /// This function does not load any game configuration as it is meant to be used to test types
    /// that load game configuration. If you want test objects and maps to be loaded, please use the
    /// `game_base` function.
    ///
    /// # Parameters
    ///
    /// * `test_name`: Name of the test, used to populate the window title.
    /// * `visibility`: Whether the window should be visible.
    pub fn render_and_ui<'name, N>(
        test_name: N,
        visibility: bool,
    ) -> AmethystApplication<GameData<'static, 'static>, ()>
    where
        N: Into<&'name str>,
    {
        AutexousiousApplication::render_base(test_name, visibility)
            .with_ui_bundles::<PlayerAxisControl, PlayerActionControl>()
    }

    /// Returns an application with game configuration loaded.
    ///
    /// This function does not instantiate any game entities. If you want test entities (objects and
    /// map) to be instantiated, please use the `game_base` function.
    ///
    /// # Parameters
    ///
    /// * `test_name`: Name of the test, used to populate the window title.
    /// * `visibility`: Whether the window should be visible.
    pub fn config_base<'name, N>(
        test_name: N,
        visibility: bool,
    ) -> AmethystApplication<GameData<'static, 'static>, ()>
    where
        N: Into<&'name str>,
    {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AutexousiousApplication::render_and_ui(test_name, visibility)
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(ObjectLoadingBundle::new())
            .with_bundle(CharacterSelectionBundle::new())
            .with_state(|| {
                LoadingState::new(
                    Path::new(env!("CARGO_MANIFEST_DIR")).join(ASSETS),
                    Box::new(EmptyState),
                )
            })
    }

    /// Returns an application with game objects loaded.
    ///
    /// TODO: Take in IDs of characters and maps to select.
    ///
    /// # Parameters
    ///
    /// * `test_name`: Name of the test, used to populate the window title.
    /// * `visibility`: Whether the window should be visible.
    pub fn game_base<'name, N>(
        test_name: N,
        visibility: bool,
    ) -> AmethystApplication<GameData<'static, 'static>, ()>
    where
        N: Into<&'name str>,
    {
        let mut character_selections = CharacterSelections::default();
        character_selections.state = CharacterSelectionsState::Ready;
        let controller_id = 0;
        let character_object_index = 0; // First loaded `Character`
        character_selections
            .selections
            .entry(controller_id)
            .or_insert(character_object_index);

        let map_selection_fn = |world: &mut World| {
            // TODO: <https://gitlab.com/azriel91/autexousious/issues/57>
            let fade_map_handle = world
                .read_resource::<Vec<MapHandle>>()
                .get(1)
                .expect("Expected at least one map to be loaded.")
                .clone();
            let map_selection =
                MapSelection::new(MapSelectionStatus::Confirmed, Some(fade_map_handle));

            world.add_resource(map_selection);
        };

        AutexousiousApplication::config_base(test_name, visibility)
            .with_resource(character_selections)
            .with_setup(map_selection_fn)
            .with_state(|| GameLoadingState::new(Box::new(|| Box::new(EmptyState))))
    }
}

#[cfg(test)]
mod test {
    use amethyst::{input::InputHandler, ui::MouseReactive};
    use amethyst_test_support::SpriteRenderAnimationFixture;
    use game_input::{PlayerActionControl, PlayerAxisControl};
    use game_model::play::GameEntities;
    use map_model::loaded::MapHandle;
    use object_model::{loaded::CharacterHandle, ObjectType};
    use strum::IntoEnumIterator;

    use super::AutexousiousApplication;

    // TODO: Allow users to specify their own type parameters to `AmethystApplication::base()`.
    //
    // This will make the dev experience better for crates that need strong types for the Input and
    // UI bundles, but are not able to depend on `AutexousiousApplication`, as the
    // `autexousious_test_support` crate would be depending on *that crate* (for better dev
    // experience for higher level crates).
    #[test]
    fn ui_base_uses_strong_types_for_input_and_ui_bundles() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::ui_base()
                .with_assertion(|world| {
                    // Panics if the type parameters used are not these ones.
                    world.read_resource::<InputHandler<PlayerAxisControl, PlayerActionControl>>();
                    world.read_storage::<MouseReactive>();
                }).run()
                .is_ok()
        );
    }

    #[test]
    fn render_base_application_can_load_sprite_render_animations() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::render_base(
                "render_base_application_can_load_sprite_render_animations",
                false
            ).with_effect(SpriteRenderAnimationFixture::effect)
            .with_assertion(SpriteRenderAnimationFixture::assertion)
            .run()
            .is_ok()
        );
    }

    #[test]
    fn render_and_ui_uses_strong_types_for_input_and_ui_bundles() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::render_and_ui(
                "render_and_ui_uses_strong_types_for_input_and_ui_bundles",
                false
            ).with_assertion(|world| {
                // Panics if the type parameters used are not these ones.
                world.read_resource::<InputHandler<PlayerAxisControl, PlayerActionControl>>();
                world.read_storage::<MouseReactive>();
            }).run()
            .is_ok()
        );
    }

    #[test]
    fn config_base_loads_assets_from_self_crate_directory() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::config_base(
                "config_base_loads_assets_from_self_crate_directory",
                false
            ).with_assertion(|world| {
                // Panics if the resources have not been populated
                world.read_resource::<Vec<MapHandle>>();
                assert!(!world.read_resource::<Vec<CharacterHandle>>().is_empty());
            }).run()
            .is_ok()
        );
    }

    #[test]
    fn game_base_loads_object_and_map_entities() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AutexousiousApplication::game_base("game_base_loads_object_and_map_entities", false)
                .with_assertion(|world| {
                    let game_entities = &*world.read_resource::<GameEntities>();

                    // Ensure there is at least one entity per object type.
                    ObjectType::iter().for_each(|object_type| {
                        assert!(
                            game_entities.objects.get(&object_type).is_some(),
                            // kcov-ignore-start
                            format!(
                                // kcov-ignore-end
                                "Expected at least one entity for the `{}` object type",
                                object_type
                            )
                        );
                    });

                    // Ensure there is at least one map layer (map is loaded).
                    assert!(
                        !game_entities.map_layers.is_empty(),
                        "Expected map to be loaded."
                    );
                }).run()
                .is_ok()
        );
    }
}
