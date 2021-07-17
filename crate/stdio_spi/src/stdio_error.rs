#![allow(clippy::nonstandard_macro_braces)] // TODO: Pending https://github.com/rust-lang/rust-clippy/issues/7434

use err_derive::Error;

/// Errors related to interaction with standard IO.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum StdioError {
    /// Message describing the error.
    #[error(display = "{}", _0)]
    Msg(String),
}
