use amethyst::{
    assets::Processor,
    audio::Source,
    core::TransformBundle,
    renderer::{types::DefaultBackend, RenderEmptyBundle},
    window::ScreenDimensions,
    GameData, LogLevelFilter, LoggerConfig,
};
use amethyst_test::{AmethystApplication, PopState, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
use application_event::{AppEvent, AppEventReader};
use asset_model::config::AssetType;
use assets_test::{ASSETS_PATH, MAP_FADE_SLUG};
use audio_loading::AudioLoadingBundle;
use background_loading::BackgroundLoadingBundle;
use character_loading::CharacterLoadingBundle;
use character_selection::CharacterSelectionBundle;
use character_selection_model::{CharacterSelections, CharacterSelectionsStatus};
use collision_audio_loading::CollisionAudioLoadingBundle;
use collision_loading::CollisionLoadingBundle;
use energy_loading::EnergyLoadingBundle;
use game_input_model::ControlBindings;
use game_loading::GameLoadingState;
use kinematic_loading::KinematicLoadingBundle;
use loading::{LoadingBundle, LoadingState};
use map_loading::MapLoadingBundle;
use object_type::ObjectType;
use sequence_loading::SequenceLoadingBundle;
use spawn_loading::SpawnLoadingBundle;
use sprite_loading::SpriteLoadingBundle;
use ui_audio_loading::UiAudioLoadingBundle;

use crate::{AssetQueries, SetupFunction};

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

    /// Returns an application with Render, Input, and UI bundles loaded.
    ///
    /// This function does not load any game assets as it is meant to be used to test types
    /// that load game assets. If you want test objects and maps to be loaded, please use the
    /// `game_base` function.
    pub fn render_and_ui(
    ) -> AmethystApplication<GameData<'static, 'static>, AppEvent, AppEventReader> {
        amethyst::Logger::from_config(LoggerConfig::default())
            .level_for("gfx_backend_vulkan", LogLevelFilter::Warn)
            .level_for("rendy_factory", LogLevelFilter::Warn)
            .level_for("rendy_memory", LogLevelFilter::Warn)
            .level_for("rendy_graph", LogLevelFilter::Warn)
            .level_for("rendy_wsi", LogLevelFilter::Warn)
            .start();

        AmethystApplication::blank()
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_bundle(TransformBundle::new())
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_ui_bundles::<ControlBindings>()
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
    }

    /// Returns an application with game assets loaded.
    ///
    /// This function does not instantiate any game entities. If you want test entities (objects and
    /// map) to be instantiated, please use the `game_base` function.
    pub fn config_base() -> AmethystApplication<GameData<'static, 'static>, AppEvent, AppEventReader>
    {
        AutexousiousApplication::render_and_ui()
            // On Windows, using `AudioBundle` causes a segfault.
            // On Linux, using `AudioSystem` (which needs a default `Output` device) causes a panic.
            //
            // Our workaround is to just include the `Source` processor as that is what's needed to
            // load the audio files.
            //
            // .with_bundle(AudioBundle::default())
            .with_system(Processor::<Source>::new(), "source_processor", &[])
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(AudioLoadingBundle::new())
            .with_bundle(KinematicLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(SpawnLoadingBundle::new())
            .with_bundle(BackgroundLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(EnergyLoadingBundle::new())
            .with_bundle(CollisionAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(UiAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CharacterSelectionBundle::new())
            .with_state(|| LoadingState::new(PopState))
    }

    /// Returns an application with game objects loaded.
    ///
    /// TODO: Take in IDs of characters and maps to select.
    pub fn game_base() -> AmethystApplication<GameData<'static, 'static>, AppEvent, AppEventReader>
    {
        AutexousiousApplication::config_base()
            .with_effect(|world| {
                let mut character_selections = CharacterSelections::default();
                let controller_id = 0;
                character_selections
                    .selections
                    .entry(controller_id)
                    .or_insert_with(|| {
                        AssetQueries::first_id(world, AssetType::Object(ObjectType::Character))
                    });

                world.insert(character_selections);
                world.insert(CharacterSelectionsStatus::Ready);
            })
            .with_effect(SetupFunction::map_selection(MAP_FADE_SLUG.clone()))
            .with_state(|| GameLoadingState::new(|| Box::new(PopState)))
    }
}
