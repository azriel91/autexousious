use std::env;

use amethyst::{audio::AudioBundle, core::transform::TransformBundle, GameData};
use amethyst_test::{AmethystApplication, PopState};
use application_event::{AppEvent, AppEventReader};
use asset_model::loaded::SlugAndHandle;
use assets_test::{ASSETS_CHAR_BAT_SLUG, ASSETS_MAP_FADE_SLUG, ASSETS_PATH};
use character_loading::CharacterLoadingBundle;
use character_selection::CharacterSelectionBundle;
use character_selection_model::{CharacterSelections, CharacterSelectionsStatus};
use collision_audio_loading::CollisionAudioLoadingBundle;
use collision_loading::CollisionLoadingBundle;
use game_input_model::ControlBindings;
use game_loading::GameLoadingState;
use loading::{LoadingBundle, LoadingState};
use map_loading::MapLoadingBundle;
use sequence_loading::SequenceLoadingBundle;
use sprite_loading::SpriteLoadingBundle;
use ui_audio_loading::UiAudioLoadingBundle;

use crate::SetupFunction;

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
    pub fn ui_base() -> AmethystApplication<GameData<'static, 'static>, AppEvent, AppEventReader> {
        AmethystApplication::ui_base::<ControlBindings>()
            .with_custom_event_type::<AppEvent, AppEventReader>()
    }

    /// Returns an application with the Animation, Transform, and Render bundles.
    ///
    /// The difference between this and `AmethystApplication::render_base()` is the type parameters
    /// to the Input and UI bundles are the `PlayerAxisControl` and `PlayerActionControl`, and there
    /// are no animation bundles.
    ///
    /// # Parameters
    ///
    /// * `test_name`: Name of the test, used to populate the window title.
    /// * `visibility`: Whether the window should be visible.
    pub fn render_base<'name, N>(
        test_name: N,
        visibility: bool,
    ) -> AmethystApplication<GameData<'static, 'static>, AppEvent, AppEventReader>
    where
        N: Into<&'name str>,
    {
        // Unfortunately we cannot re-use `AmethystApplication::render_base` because we need to
        // specify the `TransformBundle`'s dependencies.
        AmethystApplication::blank()
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_bundle(TransformBundle::new())
            .with_bundle(CollisionLoadingBundle::new())
            .with_render_bundle(test_name, visibility)
    }

    /// Returns an application with Render, Input, and UI bundles loaded.
    ///
    /// This function does not load any game assets as it is meant to be used to test types
    /// that load game assets. If you want test objects and maps to be loaded, please use the
    /// `game_base` function.
    ///
    /// # Parameters
    ///
    /// * `test_name`: Name of the test, used to populate the window title.
    /// * `visibility`: Whether the window should be visible.
    pub fn render_and_ui<'name, N>(
        test_name: N,
        visibility: bool,
    ) -> AmethystApplication<GameData<'static, 'static>, AppEvent, AppEventReader>
    where
        N: Into<&'name str>,
    {
        AutexousiousApplication::render_base(test_name, visibility)
            .with_ui_bundles::<ControlBindings>()
    }

    /// Returns an application with game assets loaded.
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
    ) -> AmethystApplication<GameData<'static, 'static>, AppEvent, AppEventReader>
    where
        N: Into<&'name str>,
    {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AutexousiousApplication::render_and_ui(test_name, visibility)
            .with_bundle(AudioBundle::default())
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(CollisionAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(UiAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CharacterSelectionBundle::new())
            .with_state(|| LoadingState::new(PopState))
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
    ) -> AmethystApplication<GameData<'static, 'static>, AppEvent, AppEventReader>
    where
        N: Into<&'name str>,
    {
        AutexousiousApplication::config_base(test_name, visibility)
            .with_setup(|world| {
                let mut character_selections = CharacterSelections::default();
                let controller_id = 0;
                character_selections
                    .selections
                    .entry(controller_id)
                    .or_insert_with(|| {
                        SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()))
                    });

                world.add_resource(character_selections);
                world.add_resource(CharacterSelectionsStatus::Ready);
            })
            .with_setup(SetupFunction::map_selection(ASSETS_MAP_FADE_SLUG.clone()))
            .with_state(|| GameLoadingState::new(|| Box::new(PopState)))
    }
}

#[cfg(test)]
mod test {
    use amethyst::{input::InputHandler, ui::Interactable, Error};
    use game_input_model::ControlBindings;
    use game_model::{
        loaded::{CharacterAssets, MapAssets},
        play::GameEntities,
    };
    use object_model::ObjectType;
    use strum::IntoEnumIterator;

    use super::AutexousiousApplication;

    #[test]
    fn ui_base_uses_strong_types_for_input_and_ui_bundles() -> Result<(), Error> {
        AutexousiousApplication::ui_base()
            .with_assertion(|world| {
                // Panics if the type parameters used are not these ones.
                world.read_resource::<InputHandler<ControlBindings>>();
                world.read_storage::<Interactable>();
            })
            .run()
    }

    #[test]
    fn render_and_ui_uses_strong_types_for_input_and_ui_bundles() -> Result<(), Error> {
        AutexousiousApplication::render_and_ui(
            "render_and_ui_uses_strong_types_for_input_and_ui_bundles",
            false,
        )
        .with_assertion(|world| {
            // Panics if the type parameters used are not these ones.
            world.read_resource::<InputHandler<ControlBindings>>();
            world.read_storage::<Interactable>();
        })
        .run()
    }

    #[test]
    fn config_base_loads_assets_from_self_crate_directory() -> Result<(), Error> {
        AutexousiousApplication::config_base(
            "config_base_loads_assets_from_self_crate_directory",
            false,
        )
        .with_assertion(|world| {
            // Panics if the resources have not been populated
            world.read_resource::<MapAssets>();
            assert!(!world.read_resource::<CharacterAssets>().is_empty());
        })
        .run()
    }

    #[test]
    fn game_base_loads_object_and_map_entities() -> Result<(), Error> {
        AutexousiousApplication::game_base("game_base_loads_object_and_map_entities", false)
            .with_assertion(|world| {
                let game_entities = &*world.read_resource::<GameEntities>();

                // Ensure there is at least one entity per object type.
                ObjectType::iter().for_each(|object_type| {
                    let objects = game_entities.objects.get(&object_type);
                    let object_entities = objects.unwrap_or_else(|| {
                        // kcov-ignore-start
                        panic!("Expected entry for the `{}` object type.", object_type)
                        // kcov-ignore-end
                    });

                    assert!(
                        !object_entities.is_empty(),
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
            })
            .run()
    }
}
