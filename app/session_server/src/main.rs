use std::{fs::File, io::BufReader, path::PathBuf};

use amethyst::{utils::application_root_dir, Error, LoggerConfig};
use structopt::StructOpt;

/// Default file for logger configuration.
const LOGGER_CONFIG: &str = "logger.yaml";

/// Options to initialize the session server.
#[derive(StructOpt, Debug)]
#[structopt(name = "Will Session Server", rename_all = "snake_case")]
pub struct Opt {
    /// Logger configuration file.
    #[structopt(long)]
    logger_config: Option<PathBuf>,
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

        eprintln!(
            "Loaded logger config from `{}`.",
            logger_config_path.display()
        );

        Ok(logger_config)
    } else if is_user_specified {
        let message = format!(
            "Warning: Failed to read logger configuration file: `{}`.",
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

    Ok(())
}
