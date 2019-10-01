use std::io;

use amethyst::{self, config::ConfigError, core};
use derive_error_chain::ErrorChain;
use error_chain;

use crate::{self, dir::DiscoveryContext};

// kcov-ignore-start
/// `ErrorKind` for application directories.
#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    /// Plain error message without additional structure or context
    Msg(String),

    /// Error when unable to find a directory beside the executable.
    #[error_chain(custom, display = r#"|e| write!(f, "{}", e)"#)]
    Discovery(DiscoveryContext),

    /// Resource error, used when searching for a sibling directory
    #[error_chain(link = "Error")]
    Resource(ErrorKind),

    /// Application configuration error due to an IO failure
    #[error_chain(foreign, display = r#"|e| write!(f, "io::Error: '{}'", e)"#)]
    Io(io::Error),
}
// kcov-ignore-end

impl From<DiscoveryContext> for Error {
    fn from(discovery_context: DiscoveryContext) -> Error {
        Error(
            ErrorKind::Discovery(discovery_context),
            error_chain::State::default(),
        ) // kcov-ignore
    }
}

impl From<Error> for io::Error {
    fn from(dir_error: Error) -> io::Error {
        match dir_error.0 /* error_kind */ {
            ErrorKind::Msg(msg) => io::Error::new(io::ErrorKind::Other, msg),
            ErrorKind::Discovery(discovery_context) => {
                io::Error::new(io::ErrorKind::Other, format!("{}", discovery_context))
            }
            ErrorKind::Resource(resource_error_kind) => {
                io::Error::new(io::ErrorKind::Other, format!("{}", resource_error_kind))
            }
            ErrorKind::Io(io_error) => io_error,
        }
    }
}

impl From<Error> for amethyst::Error {
    fn from(dir_error: Error) -> amethyst::Error {
        let dir_error = ConfigError::File(dir_error.into());
        amethyst::Error::Config(dir_error)
    }
}

impl From<Error> for core::Error {
    fn from(dir_error: Error) -> core::Error {
        core::Error::from_kind(core::ErrorKind::Msg(format!("{}", dir_error)))
    }
}
