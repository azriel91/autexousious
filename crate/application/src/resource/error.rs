use std::io;

use amethyst;
use amethyst::config::ConfigError;
use amethyst::core;
use error_chain;
use ron;

use resource::FindContext;
use resource::dir;

// kcov-ignore-start
/// `ErrorKind` for application configuration
#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    /// Plain error message without additional structure or context
    Msg(String),

    /// Error when unable to find a directory beside the executable.
    #[error_chain(custom, display = r#"|e| write!(f, "{}", e)"#)]
    DirDiscovery(dir::DiscoveryContext),

    /// Error when failing to find a configuration file
    #[error_chain(custom, display = r#"|e| write!(f, "{}", e)"#)]
    Find(FindContext),

    /// Application configuration error due to an IO failure
    #[error_chain(foreign, display = r#"|e| write!(f, "io::Error: '{}'", e)"#)]
    Io(io::Error),

    /// Error when failing to find a configuration file
    #[error_chain(foreign, display = r#"|e| write!(f, "ron::de::Error: '{}'", e)"#)]
    RonDeserialization(ron::de::Error),
}
// kcov-ignore-end

impl From<FindContext> for Error {
    fn from(find_context: FindContext) -> Error {
        Error(ErrorKind::Find(find_context), error_chain::State::default())
    }
}

impl From<dir::DiscoveryContext> for Error {
    fn from(discovery_context: dir::DiscoveryContext) -> Error {
        Error(
            ErrorKind::DirDiscovery(discovery_context),
            error_chain::State::default(),
        ) // kcov-ignore
    }
}

impl From<Error> for io::Error {
    fn from(resource_error: Error) -> io::Error {
        match resource_error.0 /* error_kind */ {
            ErrorKind::Msg(msg) => io::Error::new(io::ErrorKind::Other, msg),
            ErrorKind::DirDiscovery(discovery_context) => {
                io::Error::new(io::ErrorKind::Other, format!("{}", discovery_context))
            }
            ErrorKind::Find(find_context) => {
                io::Error::new(io::ErrorKind::Other, format!("{}", find_context))
            }
            ErrorKind::Io(io_error) => io_error,
            ErrorKind::RonDeserialization(ron_de_error) => io::Error::new(io::ErrorKind::Other, format!("{}", ron_de_error)),
        }
    }
}

impl From<Error> for amethyst::Error {
    fn from(resource_error: Error) -> amethyst::Error {
        let resource_error = ConfigError::File(resource_error.into());
        amethyst::Error::Config(resource_error)
    }
}

impl From<Error> for core::Error {
    fn from(resource_error: Error) -> core::Error {
        core::Error::from_kind(core::ErrorKind::Msg(format!("{}", resource_error)))
    }
}
