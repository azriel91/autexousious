//! Contains modules for various application configuration

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

// Must be after `mod find;` as it uses the `development_base_dirs` macro.
pub mod dir;

pub use self::error::{Error, ErrorKind};
pub use self::find::{find, find_in};
pub use self::find_context::FindContext;
pub use self::format::Format;
pub use self::load::{load, load_in};
