//! Contains modules for various application configuration

pub mod dir;

mod error;
mod find;
mod find_context;

pub use self::error::{Error, ErrorKind};
pub use self::find::{find, find_in};
pub use self::find_context::FindContext;
