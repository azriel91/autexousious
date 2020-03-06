use std::{any, fs::File, io::BufReader, net::IpAddr, path::PathBuf};

use amethyst::{
    network::simulation::laminar::{LaminarNetworkBundle, LaminarSocket},
    utils::application_root_dir,
    Application, Error, GameDataBuilder, LoggerConfig, SimpleState,
};
use frame_rate::strategy::frame_rate_limit_config;
use net_play::{NetListenerSystem, NetListenerSystemDesc};
use structopt::StructOpt;

use crate::system::{
    SessionHostResponderSystem, SessionHostResponderSystemDesc, SessionJoinResponderSystem,
    SessionJoinResponderSystemDesc,
};

pub mod model;
pub mod play;

mod system;

/// Default file for logger configuration.
const LOGGER_CONFIG: &str = "logger.yaml";

/// Default empty state
pub struct RunState;
impl SimpleState for RunState {}

/// Options to initialize the session server.
#[derive(StructOpt, Debug)]
#[structopt(name = "Will Session Server", rename_all = "snake_case")]
pub struct Opt {
    /// Logger configuration file.
    #[structopt(long)]
    logger_config: Option<PathBuf>,
    /// Frame rate to run the server at.
    #[structopt(long)]
    frame_rate: Option<u32>,

    /// Address to bind to.
    #[structopt(long, default_value = "127.0.0.1")]
    address: IpAddr,
    /// Port that the session server is listening on.
    #[structopt(long, default_value = "1234")]
    port: u16,
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

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    logger_setup(opt.logger_config)?;

    let socket = LaminarSocket::bind((opt.address, opt.port))?;

    let assets_dir = application_root_dir()?.join("./");

    let game_data = GameDataBuilder::default()
        .with_bundle(LaminarNetworkBundle::new(Some(socket)))?
        .with_system_desc(
            NetListenerSystemDesc::default(),
            any::type_name::<NetListenerSystem>(),
            &["network_recv"],
        )
        .with_system_desc(
            SessionHostResponderSystemDesc::default(),
            any::type_name::<SessionHostResponderSystem>(),
            &[any::type_name::<NetListenerSystem>()],
        )
        .with_system_desc(
            SessionJoinResponderSystemDesc::default(),
            any::type_name::<SessionJoinResponderSystem>(),
            &[any::type_name::<NetListenerSystem>()],
        );

    let mut game = Application::build(assets_dir, RunState)?
        .with_frame_limit_config(frame_rate_limit_config(opt.frame_rate))
        .build(game_data)?;
    game.run();

    Ok(())
}
