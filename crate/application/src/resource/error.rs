use std::io;

use amethyst::{self, config::ConfigError, core};
use error_chain;
use ron;
use toml;

use crate::resource::FindContext;

// kcov-ignore-start
/// `ErrorKind` for application configuration
#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    /// Plain error message without additional structure or context
    Msg(String),

    /// Error when failing to find a configuration file
    #[error_chain(custom, display = r#"|e| write!(f, "{}", e)"#)]
    Find(FindContext),

    /// Application configuration error due to an IO failure
    #[error_chain(foreign, display = r#"|e| write!(f, "io::Error: `{}`", e)"#)]
    Io(io::Error),

    /// Error when failing to find a configuration file
    #[error_chain(foreign, display = r#"|e| write!(f, "ron::de::Error: `{}`", e)"#)]
    RonDeserialization(ron::de::Error),

    /// Error when failing to find a configuration file
    #[error_chain(foreign, display = r#"|e| write!(f, "toml::de::Error: `{}`", e)"#)]
    TomlDeserialization(toml::de::Error),
}
// kcov-ignore-end

impl From<FindContext> for Error {
    fn from(find_context: FindContext) -> Error {
        Error(ErrorKind::Find(find_context), error_chain::State::default())
    }
}

impl From<Error> for io::Error {
    fn from(resource_error: Error) -> io::Error {
        match resource_error.0 /* error_kind */ {
            ErrorKind::Msg(msg) => io::Error::new(io::ErrorKind::Other, msg),
            ErrorKind::Find(find_context) => {
                io::Error::new(io::ErrorKind::Other, format!("{}", find_context))
            }
            ErrorKind::Io(io_error) => io_error,
            ErrorKind::RonDeserialization(ron_de_error) => io::Error::new(io::ErrorKind::Other, format!("{}", ron_de_error)),
            ErrorKind::TomlDeserialization(toml_de_error) => io::Error::new(io::ErrorKind::Other, format!("{}", toml_de_error)),
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

#[cfg(test)]
mod test {
    use std::{io, path::PathBuf};

    use ron;
    use toml;

    use super::{Error, ErrorKind};
    use crate::FindContext;

    #[test]
    fn msg_error_into_io_error() {
        let error = Error::from(ErrorKind::Msg("boo".to_string()));
        let io_error: io::Error = error.into();

        assert_eq!(io::ErrorKind::Other, io_error.kind());
        assert!(format!("{}", io_error).contains("boo"));
    }

    #[test]
    fn find_error_into_io_error() {
        let base_dirs = vec![];
        let conf_dir = PathBuf::new();
        let file_name = "rara".to_string();
        let find_context = FindContext {
            base_dirs,
            conf_dir,
            file_name,
        };
        let error = Error::from(ErrorKind::Find(find_context));
        let io_error: io::Error = error.into();

        assert_eq!(io::ErrorKind::Other, io_error.kind());
        assert!(format!("{}", io_error).contains("rara"));
    }

    #[test]
    fn io_error_into_io_error() {
        let error = Error::from(ErrorKind::Io(io::Error::new(
            io::ErrorKind::WriteZero,
            "boo",
        )));
        let io_error: io::Error = error.into();

        assert_eq!(io::ErrorKind::WriteZero, io_error.kind());
        assert!(format!("{}", io_error).contains("boo"));
    }

    #[test]
    fn ron_deserialization_error_into_io_error() {
        let ron_error = ron::de::from_str::<Data>(r#"Data(val: "boo")"#).unwrap_err();
        let error = Error::from(ErrorKind::RonDeserialization(ron_error));
        let io_error: io::Error = error.into();

        assert_eq!(io::ErrorKind::Other, io_error.kind());
        assert!(format!("{}", io_error).contains("Expected integer"));
    }

    #[test]
    fn toml_deserialization_error_into_io_error() {
        let toml_error = toml::from_str::<Data>(r#"val = "boo""#).unwrap_err();
        let error = Error::from(ErrorKind::TomlDeserialization(toml_error));
        let io_error: io::Error = error.into();

        assert_eq!(io::ErrorKind::Other, io_error.kind());
        assert!(format!("{}", io_error).contains("boo"));
    }

    #[derive(Debug, Deserialize)]
    struct Data {
        val: u32, // kcov-ignore
    }
}
