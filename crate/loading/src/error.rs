use std::io;

use toml;

// kcov-ignore-start
/// `ErrorKind` for loading operations.
#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    /// Plain error message without additional structure or context
    Msg(String),

    /// Application configuration error due to an IO failure
    #[error_chain(foreign, display = r#"|e| write!(f, "io::Error: '{}'", e)"#)]
    Io(io::Error),

    /// Error when failing to find a configuration file
    #[error_chain(foreign, display = r#"|e| write!(f, "toml::de::Error: '{}'", e)"#)]
    TomlDeserialization(toml::de::Error),
}
// kcov-ignore-end

impl From<Error> for io::Error {
    fn from(resource_error: Error) -> io::Error {
        match resource_error.0 /* error_kind */ {
            ErrorKind::Msg(msg) => io::Error::new(io::ErrorKind::Other, msg),
            ErrorKind::Io(io_error) => io_error,
            ErrorKind::TomlDeserialization(toml_de_error) => io::Error::new(io::ErrorKind::Other, format!("{}", toml_de_error)),
        }
    }
}
