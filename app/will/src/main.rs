#![windows_subsystem = "windows"]

use std::{
    any,
    convert::TryFrom,
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
};
#[cfg(not(target_arch = "wasm32"))]
use std::{fs::File, io::BufReader};

use amethyst::{
    assets::{HotReloadBundle, HotReloadStrategy},
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{Bindings, InputBundle},
    network::simulation::web_socket::WebSocketNetworkBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        rendy::hal::command::ClearColor,
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, ortho_camera::CameraOrthoSystem},
    window::EventLoop,
    CoreApplication, Error, GameDataBuilder,
};
#[cfg(not(target_arch = "wasm32"))]
use amethyst::{window::DisplayConfig, LoggerConfig};
use application::AppDir;
#[cfg(not(target_arch = "wasm32"))]
use application::{AppFile, Format, IoUtils};
use application_event::{AppEvent, AppEventReader, AppEventVariant};
use application_robot::RobotState;
#[cfg(not(target_arch = "wasm32"))]
use application_ui::FontConfigLoader;
use application_ui::{ApplicationUiBundle, FontConfig};
use asset_play::{AssetPlayBundle, ItemIdEventSystem};
#[cfg(not(target_arch = "wasm32"))]
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
    ControllerInputUpdateSystem, GameInputToControlInputSystem, GameInputToControlInputSystemDesc,
    InputToGameInputSystem, InputToGameInputSystemDesc, SharedControllerInputUpdateSystem,
};
use game_input_model::{
    config::{ControlBindings, PlayerInputConfigs},
    loaded::PlayerControllers,
};
use game_input_stdio::ControlInputEventStdinMapper;
use game_mode_selection::{GameModeSelectionStateBuilder, GameModeSelectionStateDelegate};
#[cfg(not(target_arch = "wasm32"))]
use game_mode_selection_stdio::GameModeSelectionStdioBundle;
use game_mode_selection_ui::GameModeSelectionSfxSystem;
use game_play::GamePlayBundle;
#[cfg(not(target_arch = "wasm32"))]
use game_play_stdio::GamePlayStdioBundle;
use input_reaction_loading::InputReactionLoadingBundle;
use kinematic_loading::KinematicLoadingBundle;
use loading::{LoadingBundle, LoadingState};
#[cfg(not(target_arch = "wasm32"))]
use log::debug;
use map_loading::MapLoadingBundle;
use net_play::{
    NetListenerSystem, NetListenerSystemDesc, NetMessageRequestSystem, NetMessageRequestSystemDesc,
};
use network_input_play::{
    GameInputTickRequestSystem, NetworkInputRequestSystem, NetworkInputRequestSystemDesc,
    NetworkInputResponseSystem, NetworkInputResponseSystemDesc,
};
#[cfg(not(target_arch = "wasm32"))]
use network_mode_selection_stdio::NetworkModeSelectionStdioBundle;
use network_session_model::config::SessionServerConfig;
use network_session_play::{
    SessionInputResourcesSyncSystem, SessionInputResourcesSyncSystemDesc,
    SessionMessageResponseSystem, SessionMessageResponseSystemDesc, SessionStatusNotifierSystem,
};
use parent_play::ChildEntityDeleteSystem;
use sequence_loading::SequenceLoadingBundle;
use serde::{Deserialize, Serialize};
use session_host_play::{
    SessionHostRequestSystem, SessionHostRequestSystemDesc, SessionHostResponseSystem,
    SessionHostResponseSystemDesc,
};
#[cfg(not(target_arch = "wasm32"))]
use session_host_stdio::SessionHostStdioBundle;
use session_join_play::{
    SessionJoinRequestSystem, SessionJoinRequestSystemDesc, SessionJoinResponseSystem,
    SessionJoinResponseSystemDesc,
};
#[cfg(not(target_arch = "wasm32"))]
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
#[cfg(not(target_arch = "wasm32"))]
use stdio_command_stdio::{StdioCommandProcessingSystem, StdioCommandStdioBundle};
#[cfg(not(target_arch = "wasm32"))]
use stdio_input::StdioInputBundle;
use stdio_spi::MapperSystem;
use structopt::StructOpt;
use structopt_toml::StructOptToml;
use tracker::PrevTrackerSystem;
use ui_audio_loading::UiAudioLoadingBundle;
use ui_loading::UiLoadingBundle;
use ui_play::{
    UiActiveWidgetUpdateSystem, UiTextColourUpdateSystem, UiTransformForFovSystem,
    UiTransformForFovSystemDesc, UiTransformInsertionRectifySystem,
    UiTransformInsertionRectifySystemDesc, WidgetSequenceUpdateSystem,
};

#[cfg(target_arch = "wasm32")]
mod built_in;

/// Default file for application arguments.
#[cfg(not(target_arch = "wasm32"))]
const WILL_CONFIG: &str = "will.toml";

/// Default file for logger configuration.
#[cfg(not(target_arch = "wasm32"))]
const LOGGER_CONFIG: &str = "logger.yaml";

/// Startup parameters for `Will`.
///
/// Note: `StructOptToml` implements `Default` for this.
#[derive(Debug, Deserialize, Serialize, StructOpt, StructOptToml)]
#[serde(default)]
#[structopt(name = "Will", rename_all = "snake_case")]
pub struct WillConfig {
    /// Frame rate to run the game at.
    #[structopt(long)]
    frame_rate: Option<u32>,
    /// Run headlessly (no GUI).
    #[serde(default)]
    #[structopt(long)]
    headless: bool,
    /// Logger configuration file.
    #[structopt(long)]
    logger_config: Option<PathBuf>,
    /// Address of the session server.
    ///
    /// Currently must be an `IpAddr`, in the future we may accept hostnames.
    #[serde(default = "WillConfig::session_server_address_default")]
    #[structopt(long, default_value = "127.0.0.1")]
    session_server_address: IpAddr,
    /// Port that the session server is listening on.
    #[serde(default = "WillConfig::session_server_port_default")]
    #[structopt(long, default_value = "1234")]
    session_server_port: u16,
}

impl WillConfig {
    fn session_server_address_default() -> IpAddr {
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    }

    fn session_server_port_default() -> u16 {
        1234
    }
}

#[cfg(not(target_arch = "wasm32"))]
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

fn session_server_config(will_config: &WillConfig) -> SessionServerConfig {
    SessionServerConfig {
        address: will_config.session_server_address,
        port: will_config.session_server_port,
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Error> {
    let mut will_config = AppFile::find(WILL_CONFIG)
        .and_then(|will_config_path| IoUtils::read_file(&will_config_path).map_err(Error::from))
        .and_then(|bytes| String::from_utf8(bytes).map_err(Error::from))
        .and_then(|will_config_toml| {
            WillConfig::from_args_with_toml(&will_config_toml).map_err(|e| Error::from(e.compat()))
        })
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            WillConfig::from_args()
        });

    logger_setup(will_config.logger_config.take())?;

    debug!("will_config: {:?}", will_config);

    let fn_setup = |_app_root: &Path, event_loop: &EventLoop<()>| {
        let player_input_configs = AppFile::load_in::<PlayerInputConfigs, _>(
            AppDir::RESOURCES,
            "player_input_configs.yaml",
            Format::Yaml,
        )?;

        let display_config = AppFile::load_in::<DisplayConfig, _>(
            AppDir::RESOURCES,
            "display_config.ron",
            Format::Ron,
        )?;
        let rendering_bundle = RenderingBundle::<DefaultBackend>::new(display_config, event_loop);

        Ok((
            will_config,
            player_input_configs,
            FontConfigLoader::load()?,
            HotReloadStrategy::default(),
            rendering_bundle,
        ))
    };

    run_application(fn_setup)
}

#[allow(unused)]
#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use std::{io::BufReader, path::Path};

    use amethyst::{
        assets::HotReloadStrategy,
        renderer::{types::DefaultBackend, RenderingBundle},
        window::{DisplayConfig, EventLoop},
        Error, LoggerConfig,
    };
    use application::{AppFile, Format};
    use application_ui::FontConfigLoader;
    use game_input_model::config::PlayerInputConfigs;
    use log::{debug, error};
    use wasm_bindgen::prelude::*;
    use web_sys::HtmlCanvasElement;

    use super::WillConfig;
    use crate::built_in::BuiltIn;

    /// Will application builder.
    #[wasm_bindgen]
    #[derive(Debug, Default)]
    pub struct WillAppBuilder {
        /// User supplied canvas, if any.
        canvas_element: Option<HtmlCanvasElement>,
        /// Will configuration.
        will_config: Option<String>,
        /// Input bindings data.
        player_input_configs: Option<String>,
        /// Theme data.
        theme: Option<String>,
        /// Logger configuration.
        logger_config: Option<String>,
    }

    #[wasm_bindgen]
    impl WillAppBuilder {
        /// Returns a new `WillAppBuilder`.
        pub fn new() -> Self {
            Self::default()
        }

        /// Sets the canvas element for the `WillAppBuilder`.
        pub fn with_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
            self.canvas_element = Some(canvas);
            self
        }

        /// Sets the `WillConfig` for the `WillAppBuilder`.
        pub fn with_will_config(mut self, will_config: String) -> Self {
            self.will_config = Some(will_config);
            self
        }

        /// Sets the `PlayerInputConfigs` configuration for the `WillAppBuilder`.
        pub fn with_player_input_configs(mut self, player_input_configs: String) -> Self {
            self.player_input_configs = Some(player_input_configs);
            self
        }

        /// Sets the `Theme` configuration for the `WillAppBuilder`.
        pub fn with_theme(mut self, theme: String) -> Self {
            self.theme = Some(theme);
            self
        }

        /// Sets the logger configuration for the `WillAppBuilder`.
        pub fn with_logger_config(mut self, logger_config: String) -> Self {
            self.logger_config = Some(logger_config);
            self
        }

        pub fn run(self) {
            // Make panic return a stack trace
            crate::init_panic_hook();

            let logger_config: LoggerConfig = self
                .logger_config
                .as_ref()
                .map(String::as_bytes)
                .map(BufReader::new)
                .map(serde_yaml::from_reader)
                .map(Result::ok)
                .flatten()
                .unwrap_or_default();
            amethyst::Logger::from_config(logger_config).start();

            debug!("canvas element: {:?}", self.canvas_element);

            let dimensions = self
                .canvas_element
                .as_ref()
                .map(|canvas_element| (canvas_element.width(), canvas_element.height()));
            debug!("dimensions: {:?}", dimensions);

            let display_config = DisplayConfig {
                dimensions,
                ..Default::default()
            };

            let setup_fn = move |_: &Path, event_loop: &EventLoop<()>| {
                let will_config = if let Some(will_config) = self.will_config.as_ref() {
                    AppFile::load_bytes(will_config.as_bytes(), Format::Yaml)?
                } else {
                    WillConfig::default()
                };

                let player_input_configs =
                    if let Some(player_input_configs) = self.player_input_configs.as_ref() {
                        AppFile::load_bytes(player_input_configs.as_bytes(), Format::Yaml)?
                    } else {
                        // Hard coded player_input_configs
                        debug!("Using built in player_input_configs.");

                        PlayerInputConfigs::built_in()
                    };

                let font_config = if let Some(font_config) = self.theme {
                    FontConfigLoader::load_bytes(font_config.as_bytes())
                } else {
                    Err(Error::from_string("Theme configuration not set."))
                }?;
                let rendering_bundle = RenderingBundle::<DefaultBackend>::new(
                    display_config,
                    event_loop,
                    self.canvas_element,
                );

                Ok((
                    will_config,
                    player_input_configs,
                    font_config,
                    HotReloadStrategy::every(10),
                    rendering_bundle,
                ))
            };

            match super::run_application(setup_fn) {
                Ok(_) => {}
                Err(e) => error!("Main returned an error: {:?}", e),
            }
        }
    }
}

fn run_application<FnSetup>(fn_setup: FnSetup) -> Result<(), Error>
where
    FnSetup: FnOnce(
        &Path,
        &EventLoop<()>,
    ) -> Result<
        (
            WillConfig,
            PlayerInputConfigs,
            FontConfig,
            HotReloadStrategy,
            RenderingBundle<DefaultBackend>,
        ),
        Error,
    >,
{
    let app_root = application_root_dir()?;
    let assets_dir = AppDir::assets()?;

    let event_loop = EventLoop::new();
    let (will_config, player_input_configs, font_config, hot_reload_strategy, rendering_bundle) =
        fn_setup(&app_root, &event_loop)?;

    let session_server_config = session_server_config(&will_config);

    let game_mode_selection_state =
        GameModeSelectionStateBuilder::new(GameModeSelectionStateDelegate::new()).build();
    let loading_state = LoadingState::<_>::new(game_mode_selection_state);
    let state = RobotState::new(Box::new(loading_state));

    let player_controllers = PlayerControllers::from(&player_input_configs);

    let bindings = Bindings::try_from(&player_input_configs)?;

    let mut game_data = GameDataBuilder::default();
    if !will_config.headless {
        // `InputBundle` provides `InputHandler<A, B>`, needed by the `UiBundle` for mouse events.
        // `UiBundle` registers `Loader<FontAsset>`, needed by `ApplicationUiBundle`.
        game_data = game_data
            .with_bundle(AudioBundle::default())?
            .with_bundle(InputBundle::<ControlBindings>::new().with_bindings(bindings))?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            game_data = game_data.with_bundle(WebSocketNetworkBundle::new(None))?;
        }
        #[cfg(target_arch = "wasm32")]
        {
            game_data = game_data.with_bundle(WebSocketNetworkBundle::new())?;
        }

        game_data = game_data
            .with_bundle(HotReloadBundle::new(hot_reload_strategy))?
            .with_bundle(SpriteLoadingBundle::new())?
            .with_bundle(SequenceLoadingBundle::new())?
            .with_bundle(AudioLoadingBundle::new())?
            .with_bundle(KinematicLoadingBundle::new())?
            .with_bundle(LoadingBundle::new(assets_dir.clone()))?
            .with_system_desc(
                InputToGameInputSystemDesc::default(),
                any::type_name::<InputToGameInputSystem>(),
                &["input_system"],
            )
            .with_system_desc(
                GameInputToControlInputSystemDesc::default(),
                any::type_name::<GameInputToControlInputSystem>(),
                &[any::type_name::<InputToGameInputSystem>()],
            )
            .with(
                MapperSystem::<ControlInputEventStdinMapper>::new(AppEventVariant::ControlInput),
                any::type_name::<MapperSystem<ControlInputEventStdinMapper>>(),
                // Depend on the input handler updated system, so that stdin input takes priority.
                &[any::type_name::<GameInputToControlInputSystem>()],
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
            );

        #[cfg(not(target_arch = "wasm32"))]
        {
            game_data = game_data
                .with_bundle(StdioInputBundle::new())?
                .with_bundle(StdioCommandStdioBundle::new())?
                .with_bundle(AssetSelectionStdioBundle::new())?
                .with_bundle(GamePlayStdioBundle::new())?
                .with_bundle(GameModeSelectionStdioBundle::new())?
                .with_bundle(NetworkModeSelectionStdioBundle::new())?
                .with_bundle(SessionHostStdioBundle::new())?
                .with_bundle(SessionJoinStdioBundle::new())?;
        }

        #[cfg(not(target_arch = "wasm32"))]
        let ui_active_widget_deps = [any::type_name::<StdioCommandProcessingSystem>()];
        #[cfg(target_arch = "wasm32")]
        let ui_active_widget_deps = [];

        game_data = game_data
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
                &ui_active_widget_deps,
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
            .with(
                SessionStatusNotifierSystem::new(),
                any::type_name::<SessionStatusNotifierSystem>(),
                &[],
            )
            .with_system_desc(
                SessionInputResourcesSyncSystemDesc::default(),
                any::type_name::<SessionInputResourcesSyncSystem>(),
                &[],
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
                NetworkInputRequestSystemDesc::default(),
                any::type_name::<NetworkInputRequestSystem>(),
                &["input_system"],
            )
            .with(
                GameInputTickRequestSystem::new(),
                any::type_name::<GameInputTickRequestSystem>(),
                &[any::type_name::<NetworkInputRequestSystem>()],
            )
            .with_system_desc(
                NetMessageRequestSystemDesc::default(),
                any::type_name::<NetMessageRequestSystem>(),
                &[
                    any::type_name::<SessionHostRequestSystem>(),
                    any::type_name::<SessionJoinRequestSystem>(),
                    any::type_name::<SessionLobbyRequestSystem>(),
                    any::type_name::<NetworkInputRequestSystem>(),
                    any::type_name::<GameInputTickRequestSystem>(),
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
                &[
                    any::type_name::<NetListenerSystem>(),
                    any::type_name::<GameInputTickRequestSystem>(),
                ],
            )
            .with_system_desc(
                NetworkInputResponseSystemDesc::default(),
                any::type_name::<NetworkInputResponseSystem>(),
                &[
                    any::type_name::<NetListenerSystem>(),
                    any::type_name::<SessionMessageResponseSystem>(),
                ],
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
            .with_bundle(ApplicationUiBundle::new(font_config))?
            .with_bundle(
                rendering_bundle
                    .with_plugin(RenderToWindow::new().with_clear(ClearColor {
                        float32: [0., 0., 0., 1.],
                    }))
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

    let app = CoreApplication::<_, AppEvent, AppEventReader>::build(assets_dir, state)?
        .with_resource(session_server_config)
        .with_resource(player_controllers)
        .with_resource(player_input_configs)
        .with_frame_limit_config(frame_rate_limit_config(will_config.frame_rate))
        .build(game_data)?;

    app.run_winit_loop(event_loop);
}
