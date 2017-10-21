//! Contains modules for various application configuration

mod error;
mod find;

pub use self::error::{Error, ErrorKind};

// Consumers will use this as `application::config::find()`
pub use self::find::find;
