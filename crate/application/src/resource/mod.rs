//! Contains modules for various application configuration

pub mod dir;

#[cfg(test)]
#[macro_use]
mod test_support;

mod error;
// `development_base_dirs` macro is also used in tests.
#[macro_use]
mod find;
mod find_context;
mod format;
mod io_support;
mod load;

pub use self::error::{Error, ErrorKind};
pub use self::find::{find, find_in};
pub use self::find_context::FindContext;
pub use self::format::Format;
pub use self::load::{load, load_in};
