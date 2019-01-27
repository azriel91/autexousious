use err_derive::Error;

/// Errors related to interaction with standard IO.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum StdioError {
    /// Message describing the error.
    #[error(display = "{}", _0)]
    Msg(String),
}
