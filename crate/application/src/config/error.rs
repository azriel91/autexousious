use std::io;

use error_chain;

use config::FindContext;

// kcov-ignore-start
/// ErrorKind for application configuration
#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    /// Plain error message without additional structure or context
    Msg(String),

    /// Error when failing to find a configuration file
    #[error_chain(custom, display = r#"|e| write!(f, "{}", e)"#)]
    Find(FindContext),

    /// Application configuration error due to an IO failure
    #[error_chain(foreign, display = r#"|e| write!(f, "io::Error: '{}'", e)"#)]
    Io(io::Error),
}
// kcov-ignore-end

impl Into<Error> for FindContext {
    fn into(self) -> Error {
        Error(ErrorKind::Find(self), error_chain::State::default())
    }
}
