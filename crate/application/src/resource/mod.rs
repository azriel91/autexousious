//! Contains modules for various application configuration

pub mod dir;

mod error;
mod find;
mod find_context;
mod format;
mod load;

pub use self::error::{Error, ErrorKind};
pub use self::find::{find, find_in};
pub use self::find_context::FindContext;
pub use self::format::Format;
pub use self::load::{load, load_in};
