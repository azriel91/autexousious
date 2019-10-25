#![windows_subsystem = "windows"]

use std::{convert::TryFrom, process};

use amethyst::{
    assets::HotReloadBundle,
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{Bindings, InputBundle},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::ortho_camera::CameraOrthoSystem,
    window::DisplayConfig,
    CoreApplication, GameDataBuilder, LogLevelFilter, LoggerConfig,
};
use application::{AppDir, AppFile, Format};
use application_event::{AppEvent, AppEventReader};
use application_robot::RobotState;
use audio_loading::AudioLoadingBundle;
use background_loading::BackgroundLoadingBundle;
use camera_play::CameraPlayBundle;
use character_loading::CharacterLoadingBundle;
use character_selection_stdio::CharacterSelectionStdioBundle;
use collision_audio_loading::CollisionAudioLoadingBundle;
use collision_loading::CollisionLoadingBundle;
use energy_loading::EnergyLoadingBundle;
use frame_rate::strategy::frame_rate_limit_config;
use game_input::GameInputBundle;
use game_input_model::{ControlBindings, InputConfig};
use game_input_stdio::{ControlInputEventStdinMapper, GameInputStdioBundle};
use game_input_ui::{GameInputUiBundle, InputToControlInputSystem};
use game_mode_selection::{GameModeSelectionStateBuilder, GameModeSelectionStateDelegate};
use game_mode_selection_stdio::GameModeSelectionStdioBundle;
use game_mode_selection_ui::GameModeSelectionUiBundle;
use game_play_stdio::GamePlayStdioBundle;
use kinematic_loading::KinematicLoadingBundle;
use loading::{LoadingBundle, LoadingState};
use map_loading::MapLoadingBundle;
use map_selection_stdio::MapSelectionStdioBundle;
use sequence_loading::SequenceLoadingBundle;
use spawn_loading::SpawnLoadingBundle;
use sprite_loading::SpriteLoadingBundle;
use stdio_command_stdio::StdioCommandStdioBundle;
use stdio_input::StdioInputBundle;
use stdio_spi::MapperSystem;
use structopt::StructOpt;
use typename::TypeName;
use ui_audio_loading::UiAudioLoadingBundle;

#[derive(StructOpt, Debug)]
#[structopt(name = "Will", rename_all = "snake_case")]
struct Opt {
    /// Run headlessly (no GUI).
    #[structopt(long)]
    headless: bool,
    /// Frame rate to run the game at.
    #[structopt(long)]
    frame_rate: Option<u32>,
}

fn run(opt: &Opt) -> Result<(), amethyst::Error> {
    amethyst::Logger::from_config(LoggerConfig::default())
        .level_for("gfx_backend_vulkan", LogLevelFilter::Warn)
        .level_for("rendy_factory", LogLevelFilter::Warn)
        .level_for("rendy_memory", LogLevelFilter::Warn)
        .level_for("rendy_graph", LogLevelFilter::Warn)
        .level_for("rendy_wsi", LogLevelFilter::Warn)
        .start();

    let assets_dir = AppDir::assets()?;

    let game_mode_selection_state =
        GameModeSelectionStateBuilder::new(GameModeSelectionStateDelegate::new())
            .with_bundle(GameModeSelectionUiBundle::new())
            .build();
    let loading_state = LoadingState::<_>::new(game_mode_selection_state);
    let state = RobotState::new(Box::new(loading_state));

    let mut game_data = GameDataBuilder::default();
    if !opt.headless {
        let display_config = AppFile::load_in::<DisplayConfig, _>(
            AppDir::RESOURCES,
            "display_config.ron",
            Format::Ron,
        )?;

        let input_config =
            AppFile::load_in::<InputConfig, _>(AppDir::RESOURCES, "input_config.ron", Format::Ron)?;

        // `InputBundle` provides `InputHandler<A, B>`, needed by the `UiBundle` for mouse events.
        // `UiBundle` registers `Loader<FontAsset>`, needed by `ApplicationUiBundle`.
        game_data = game_data
            .with_bundle(AudioBundle::default())?
            .with_bundle(TransformBundle::new())?
            .with_bundle(
                InputBundle::<ControlBindings>::new()
                    .with_bindings(Bindings::try_from(&input_config)?),
            )?
            .with_bundle(UiBundle::<ControlBindings>::new())?
            .with_bundle(HotReloadBundle::default())?
            .with_bundle(SpriteLoadingBundle::new())?
            .with_bundle(SequenceLoadingBundle::new())?
            .with_bundle(AudioLoadingBundle::new())?
            .with_bundle(KinematicLoadingBundle::new())?
            .with_bundle(LoadingBundle::new(assets_dir.clone()))?
            .with_bundle(GameInputUiBundle::new(input_config))?
            .with_bundle(
                GameInputStdioBundle::new()
                    // Note: Depend on the input handler updated system, so that stdin input takes
                    // priority
                    .with_system_dependencies(&[InputToControlInputSystem::type_name()]),
            )?
            .with_bundle(GameInputBundle::new().with_system_dependencies(&[
                MapperSystem::<ControlInputEventStdinMapper>::type_name(),
                InputToControlInputSystem::type_name(),
            ]))?
            .with_bundle(StdioInputBundle::new())?
            .with_bundle(StdioCommandStdioBundle::new())?
            .with_bundle(CharacterSelectionStdioBundle::new())?
            .with_bundle(GamePlayStdioBundle::new())?
            .with_bundle(GameModeSelectionStdioBundle::new())?
            .with_bundle(MapSelectionStdioBundle::new())?
            .with_bundle(CollisionLoadingBundle::new())?
            .with_bundle(SpawnLoadingBundle::new())?
            .with_bundle(BackgroundLoadingBundle::new())?
            .with_bundle(MapLoadingBundle::new())?
            .with_bundle(CharacterLoadingBundle::new())?
            .with_bundle(EnergyLoadingBundle::new())?
            .with_bundle(CollisionAudioLoadingBundle::new(assets_dir.clone()))?
            .with_bundle(UiAudioLoadingBundle::new(assets_dir.clone()))?
            .with(CameraOrthoSystem::default(), "camera_ortho", &[])
            .with_bundle(
                RenderingBundle::<DefaultBackend>::new()
                    .with_plugin(
                        RenderToWindow::from_config(display_config).with_clear([0., 0., 0., 1.0]),
                    )
                    .with_plugin(RenderFlat2D::default())
                    .with_plugin(RenderUi::default()),
            )?
            .with_bundle(CameraPlayBundle::new())?;
    }

    let mut app = CoreApplication::<_, AppEvent, AppEventReader>::build(assets_dir, state)?
        .with_frame_limit_config(frame_rate_limit_config(opt.frame_rate))
        .build(game_data)?;

    app.run();

    Ok(())
}

fn main() {
    let opt = Opt::from_args();

    if let Err(e) = run(&opt) {
        println!("Failed to execute example: {}", e);
        process::exit(1);
    }
}
