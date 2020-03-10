#![windows_subsystem = "windows"]

use std::{any, convert::TryFrom, fs::File, io::BufReader, net::IpAddr, path::PathBuf, process};

use amethyst::{
    assets::HotReloadBundle,
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{Bindings, InputBundle},
    network::simulation::tcp::TcpNetworkBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, ortho_camera::CameraOrthoSystem},
    window::DisplayConfig,
    CoreApplication, Error, GameDataBuilder, LoggerConfig,
};
use application::{AppDir, AppFile, Format};
use application_event::{AppEvent, AppEventReader, AppEventVariant};
use application_robot::RobotState;
use asset_play::{AssetPlayBundle, ItemIdEventSystem};
use asset_selection_stdio::AssetSelectionStdioBundle;
use asset_selection_ui_play::{
    ApwPreviewSpawnSystemCharacter, ApwPreviewSpawnSystemMap, AssetSelectionSfxSystem,
    AswPortraitUpdateSystem,
};
use asset_ui_play::AssetSelectionHighlightUpdateSystem;
use audio_loading::AudioLoadingBundle;
use background_loading::BackgroundLoadingBundle;
use camera_play::CameraPlayBundle;
use character_loading::CharacterLoadingBundle;
use collision_audio_loading::CollisionAudioLoadingBundle;
use collision_loading::CollisionLoadingBundle;
use energy_loading::EnergyLoadingBundle;
use frame_rate::strategy::frame_rate_limit_config;
use game_input::{
    ControllerInputUpdateSystem, InputToControlInputSystem, InputToControlInputSystemDesc,
    SharedControllerInputUpdateSystem,
};
use game_input_model::{
    config::{ControlBindings, PlayerInputConfigs},
    loaded::PlayerControllers,
};
use game_input_stdio::ControlInputEventStdinMapper;
use game_mode_selection::{GameModeSelectionStateBuilder, GameModeSelectionStateDelegate};
use game_mode_selection_stdio::GameModeSelectionStdioBundle;
use game_mode_selection_ui::GameModeSelectionSfxSystem;
use game_play::GamePlayBundle;
use game_play_stdio::GamePlayStdioBundle;
use input_reaction_loading::InputReactionLoadingBundle;
use kinematic_loading::KinematicLoadingBundle;
use loading::{LoadingBundle, LoadingState};
use map_loading::MapLoadingBundle;
use net_play::{
    NetListenerSystem, NetListenerSystemDesc, NetMessageRequestSystem, NetMessageRequestSystemDesc,
};
use network_mode_selection_stdio::NetworkModeSelectionStdioBundle;
use network_session_model::config::SessionServerConfig;
use network_session_play::{SessionMessageResponseSystem, SessionMessageResponseSystemDesc};
use parent_play::ChildEntityDeleteSystem;
use sequence_loading::SequenceLoadingBundle;
use session_host_play::{
    SessionHostRequestSystem, SessionHostRequestSystemDesc, SessionHostResponseSystem,
    SessionHostResponseSystemDesc,
};
use session_host_stdio::SessionHostStdioBundle;
use session_join_play::{
    SessionJoinRequestSystem, SessionJoinRequestSystemDesc, SessionJoinResponseSystem,
    SessionJoinResponseSystemDesc,
};
use session_join_stdio::SessionJoinStdioBundle;
use session_lobby_play::{
    SessionLobbyRequestSystem, SessionLobbyRequestSystemDesc, SessionLobbyResponseSystem,
    SessionLobbyResponseSystemDesc,
};
use session_lobby_ui_play::{
    SessionCodeLabelUpdateSystem, SessionDeviceEntityCreateDeleteSystem,
    SessionDeviceWidgetUpdateSystem,
};
use spawn_loading::SpawnLoadingBundle;
use sprite_loading::SpriteLoadingBundle;
use state_play::{
    StateCameraResetSystem, StateIdEventSystem, StateItemSpawnSystem,
    StateItemUiInputAugmentSystem, StateItemUiInputAugmentSystemDesc,
};
use state_registry::StateId;
use stdio_command_stdio::{StdioCommandProcessingSystem, StdioCommandStdioBundle};
use stdio_input::StdioInputBundle;
use stdio_spi::MapperSystem;
use structopt::StructOpt;
use tracker::PrevTrackerSystem;
use ui_audio_loading::UiAudioLoadingBundle;
use ui_loading::UiLoadingBundle;
use ui_play::{
    UiActiveWidgetUpdateSystem, UiTextColourUpdateSystem, UiTransformForFovSystem,
    UiTransformForFovSystemDesc, UiTransformInsertionRectifySystem,
    UiTransformInsertionRectifySystemDesc, WidgetSequenceUpdateSystem,
};

/// Default file for logger configuration.
const LOGGER_CONFIG: &str = "logger.yaml";

/// `TcpListener` buffer size.
const TCP_RECV_BUFFER_SIZE: usize = 2048;

#[derive(StructOpt, Debug)]
#[structopt(name = "Will", rename_all = "snake_case")]
struct Opt {
    /// Frame rate to run the game at.
    #[structopt(long)]
    frame_rate: Option<u32>,
    /// Run headlessly (no GUI).
    #[structopt(long)]
    headless: bool,
    /// Logger configuration file.
    #[structopt(long)]
    logger_config: Option<PathBuf>,
    /// Address of the session server.
    ///
    /// Currently must be an `IpAddr`, in the future we may accept hostnames.
    #[structopt(long, default_value = "127.0.0.1")]
    session_server_address: IpAddr,
    /// Port that the session server is listening on.
    #[structopt(long, default_value = "1234")]
    session_server_port: u16,
}

fn logger_setup(logger_config_path: Option<PathBuf>) -> Result<(), Error> {
    let is_user_specified = logger_config_path.is_some();

    // If the user specified a logger configuration path, use that.
    // Otherwise fallback to a default.
    let logger_config_path = logger_config_path.unwrap_or_else(|| PathBuf::from(LOGGER_CONFIG));
    let logger_config_path = if logger_config_path.is_relative() {
        let app_dir = application_root_dir()?;
        app_dir.join(logger_config_path)
    } else {
        logger_config_path
    };

    let logger_config: LoggerConfig = if logger_config_path.exists() {
        let logger_file = File::open(&logger_config_path)?;
        let mut logger_file_reader = BufReader::new(logger_file);
        let logger_config = serde_yaml::from_reader(&mut logger_file_reader)?;

        Ok(logger_config)
    } else if is_user_specified {
        let message = format!(
            "Failed to read logger configuration file: `{}`.",
            logger_config_path.display()
        );
        eprintln!("{}", message);

        Err(Error::from_string(message))
    } else {
        Ok(LoggerConfig::default())
    }?;

    amethyst::Logger::from_config(logger_config).start();

    Ok(())
}

fn session_server_config(opt: &Opt) -> SessionServerConfig {
    SessionServerConfig {
        address: opt.session_server_address,
        port: opt.session_server_port,
    }
}

fn run(opt: Opt) -> Result<(), amethyst::Error> {
    let session_server_config = session_server_config(&opt);

    logger_setup(opt.logger_config)?;

    let assets_dir = AppDir::assets()?;

    let game_mode_selection_state =
        GameModeSelectionStateBuilder::new(GameModeSelectionStateDelegate::new()).build();
    let loading_state = LoadingState::<_>::new(game_mode_selection_state);
    let state = RobotState::new(Box::new(loading_state));

    let player_input_configs = AppFile::load_in::<PlayerInputConfigs, _>(
        AppDir::RESOURCES,
        "player_input_configs.yaml",
        Format::Yaml,
    )?;
    let player_controllers = PlayerControllers::from(&player_input_configs);

    let mut game_data = GameDataBuilder::default();
    if !opt.headless {
        let display_config = AppFile::load_in::<DisplayConfig, _>(
            AppDir::RESOURCES,
            "display_config.ron",
            Format::Ron,
        )?;

        // `InputBundle` provides `InputHandler<A, B>`, needed by the `UiBundle` for mouse events.
        // `UiBundle` registers `Loader<FontAsset>`, needed by `ApplicationUiBundle`.
        game_data = game_data
            .with_bundle(AudioBundle::default())?
            .with_bundle(
                InputBundle::<ControlBindings>::new()
                    .with_bindings(Bindings::try_from(&player_input_configs)?),
            )?
            .with_bundle(TcpNetworkBundle::new(None, TCP_RECV_BUFFER_SIZE))?
            .with_bundle(HotReloadBundle::default())?
            .with_bundle(SpriteLoadingBundle::new())?
            .with_bundle(SequenceLoadingBundle::new())?
            .with_bundle(AudioLoadingBundle::new())?
            .with_bundle(KinematicLoadingBundle::new())?
            .with_bundle(LoadingBundle::new(assets_dir.clone()))?
            .with_system_desc(
                InputToControlInputSystemDesc::default(),
                any::type_name::<InputToControlInputSystem>(),
                &["input_system"],
            )
            .with(
                MapperSystem::<ControlInputEventStdinMapper>::new(AppEventVariant::ControlInput),
                any::type_name::<MapperSystem<ControlInputEventStdinMapper>>(),
                // Depend on the input handler updated system, so that stdin input takes priority.
                &[any::type_name::<InputToControlInputSystem>()],
            )
            .with(
                ControllerInputUpdateSystem::new(),
                any::type_name::<ControllerInputUpdateSystem>(),
                &[any::type_name::<MapperSystem<ControlInputEventStdinMapper>>()],
            )
            .with(
                SharedControllerInputUpdateSystem::new(),
                any::type_name::<SharedControllerInputUpdateSystem>(),
                &[any::type_name::<ControllerInputUpdateSystem>()],
            )
            .with_bundle(StdioInputBundle::new())?
            .with_bundle(StdioCommandStdioBundle::new())?
            .with_bundle(AssetSelectionStdioBundle::new())?
            .with_bundle(GamePlayStdioBundle::new())?
            .with_bundle(GameModeSelectionStdioBundle::new())?
            .with_bundle(NetworkModeSelectionStdioBundle::new())?
            .with_bundle(SessionHostStdioBundle::new())?
            .with_bundle(SessionJoinStdioBundle::new())?
            .with_bundle(CollisionLoadingBundle::new())?
            .with_bundle(SpawnLoadingBundle::new())?
            .with_bundle(BackgroundLoadingBundle::new())?
            .with_bundle(UiLoadingBundle::new())?
            .with_bundle(MapLoadingBundle::new())?
            .with_bundle(CharacterLoadingBundle::new())?
            .with_bundle(EnergyLoadingBundle::new())?
            .with_bundle(InputReactionLoadingBundle::new())?
            .with_bundle(CollisionAudioLoadingBundle::new(assets_dir.clone()))?
            .with_bundle(UiAudioLoadingBundle::new(assets_dir.clone()))?
            .with(CameraOrthoSystem::default(), "camera_ortho", &[])
            .with(
                UiActiveWidgetUpdateSystem::new(),
                any::type_name::<UiActiveWidgetUpdateSystem>(),
                &[any::type_name::<StdioCommandProcessingSystem>()],
            )
            .with(
                UiTextColourUpdateSystem::new(),
                any::type_name::<UiTextColourUpdateSystem>(),
                &[any::type_name::<UiActiveWidgetUpdateSystem>()],
            )
            .with(
                WidgetSequenceUpdateSystem::new(),
                any::type_name::<WidgetSequenceUpdateSystem>(),
                &[any::type_name::<UiActiveWidgetUpdateSystem>()],
            )
            .with(
                StateIdEventSystem::new(),
                any::type_name::<StateIdEventSystem>(),
                &[any::type_name::<UiActiveWidgetUpdateSystem>()],
            )
            .with(
                StateCameraResetSystem::new(),
                any::type_name::<StateCameraResetSystem>(),
                &[any::type_name::<StateIdEventSystem>()],
            )
            .with(
                StateItemSpawnSystem::new(),
                any::type_name::<StateItemSpawnSystem>(),
                &[any::type_name::<StateIdEventSystem>()],
            )
            .with(
                ItemIdEventSystem::new(),
                any::type_name::<ItemIdEventSystem>(),
                &[any::type_name::<StateItemSpawnSystem>()],
            )
            .with_bundle(AssetPlayBundle::new())?
            .with_system_desc(
                SessionHostRequestSystemDesc::default(),
                any::type_name::<SessionHostRequestSystem>(),
                &[],
            )
            .with_system_desc(
                SessionJoinRequestSystemDesc::default(),
                any::type_name::<SessionJoinRequestSystem>(),
                &[],
            )
            .with_system_desc(
                SessionLobbyRequestSystemDesc::default(),
                any::type_name::<SessionLobbyRequestSystem>(),
                &[],
            )
            .with_system_desc(
                NetMessageRequestSystemDesc::default(),
                any::type_name::<NetMessageRequestSystem>(),
                &[
                    any::type_name::<SessionHostRequestSystem>(),
                    any::type_name::<SessionJoinRequestSystem>(),
                    any::type_name::<SessionLobbyRequestSystem>(),
                ],
            )
            .with_system_desc(
                NetListenerSystemDesc::default(),
                any::type_name::<NetListenerSystem>(),
                &[],
            )
            .with_system_desc(
                SessionHostResponseSystemDesc::default(),
                any::type_name::<SessionHostResponseSystem>(),
                &[any::type_name::<NetListenerSystem>()],
            )
            .with_system_desc(
                SessionJoinResponseSystemDesc::default(),
                any::type_name::<SessionJoinResponseSystem>(),
                &[any::type_name::<NetListenerSystem>()],
            )
            .with_system_desc(
                SessionLobbyResponseSystemDesc::default(),
                any::type_name::<SessionLobbyResponseSystem>(),
                &[any::type_name::<NetListenerSystem>()],
            )
            .with_system_desc(
                SessionMessageResponseSystemDesc::default(),
                any::type_name::<SessionMessageResponseSystem>(),
                &[any::type_name::<NetListenerSystem>()],
            )
            .with(
                SessionCodeLabelUpdateSystem::new(),
                any::type_name::<SessionCodeLabelUpdateSystem>(),
                &[
                    any::type_name::<SessionHostResponseSystem>(),
                    any::type_name::<SessionJoinResponseSystem>(),
                    any::type_name::<SessionMessageResponseSystem>(),
                ],
            )
            .with(
                SessionDeviceEntityCreateDeleteSystem::new(),
                any::type_name::<SessionDeviceEntityCreateDeleteSystem>(),
                &[
                    any::type_name::<SessionHostResponseSystem>(),
                    any::type_name::<SessionJoinResponseSystem>(),
                    any::type_name::<SessionMessageResponseSystem>(),
                ],
            )
            .with(
                SessionDeviceWidgetUpdateSystem::new(),
                any::type_name::<SessionDeviceWidgetUpdateSystem>(),
                &[any::type_name::<SessionDeviceEntityCreateDeleteSystem>()],
            )
            .with_system_desc(
                StateItemUiInputAugmentSystemDesc::default(),
                any::type_name::<StateItemUiInputAugmentSystem>(),
                &[],
            )
            .with(
                PrevTrackerSystem::<StateId>::new(stringify!(StateId)),
                "state_id_prev_tracker_system",
                &[],
            )
            .with_barrier()
            .with_bundle(GamePlayBundle::new())?
            .with(
                GameModeSelectionSfxSystem::new(),
                any::type_name::<GameModeSelectionSfxSystem>(),
                &[],
            )
            .with(
                AssetSelectionSfxSystem::new(),
                any::type_name::<AssetSelectionSfxSystem>(),
                &[],
            )
            .with(
                AssetSelectionHighlightUpdateSystem::new(),
                any::type_name::<AssetSelectionHighlightUpdateSystem>(),
                &[],
            )
            .with(
                AswPortraitUpdateSystem::new(),
                any::type_name::<AswPortraitUpdateSystem>(),
                &[any::type_name::<AssetSelectionHighlightUpdateSystem>()],
            )
            .with(
                ApwPreviewSpawnSystemCharacter::new(),
                any::type_name::<ApwPreviewSpawnSystemCharacter>(),
                &[any::type_name::<AssetSelectionHighlightUpdateSystem>()],
            )
            .with(
                ApwPreviewSpawnSystemMap::new(),
                any::type_name::<ApwPreviewSpawnSystemMap>(),
                &[any::type_name::<AssetSelectionHighlightUpdateSystem>()],
            )
            .with(
                ChildEntityDeleteSystem::new(),
                any::type_name::<ChildEntityDeleteSystem>(),
                &[],
            )
            .with_barrier()
            // To remove the 1 frame of flicker issue, we must run `TransformSystem` near the end,
            // so that the global matrix is updated even when the local matrix is up to date.
            //
            // `UiBundle` has a hardcoded dependency on `"transform_system"`, so we have to shift it
            // down as well.
            .with_bundle(TransformBundle::new())?
            .with_bundle(UiBundle::<ControlBindings>::new())?
            .with_bundle(
                RenderingBundle::<DefaultBackend>::new()
                    .with_plugin(
                        RenderToWindow::from_config(display_config).with_clear([0., 0., 0., 1.0]),
                    )
                    .with_plugin(RenderFlat2D::default())
                    .with_plugin(RenderUi::default()),
            )?
            .with_bundle(CameraPlayBundle::new())?
            .with_system_desc(
                UiTransformForFovSystemDesc::default(),
                any::type_name::<UiTransformForFovSystem>(),
                &["camera_ortho"],
            )
            .with_system_desc(
                UiTransformInsertionRectifySystemDesc::default(),
                any::type_name::<UiTransformInsertionRectifySystem>(),
                &[any::type_name::<UiTransformForFovSystem>()],
            );
    }

    let mut app = CoreApplication::<_, AppEvent, AppEventReader>::build(assets_dir, state)?
        .with_resource(session_server_config)
        .with_resource(player_controllers)
        .with_resource(player_input_configs)
        .with_frame_limit_config(frame_rate_limit_config(opt.frame_rate))
        .build(game_data)?;

    app.run();

    Ok(())
}

fn main() {
    let opt = Opt::from_args();

    if let Err(e) = run(opt) {
        println!("Failed to execute example: {}", e);
        process::exit(1);
    }
}
