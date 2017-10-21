use std::io;

/// ErrorKind for application configuration
#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    /// Plain error message without additional structure or context
    Msg(String),

    /// Application configuration error due to an IO failure
    #[error_chain(foreign, display = r#"|e| write!(f, "io::Error: '{}'", e)"#)]
    Io(io::Error),
}
