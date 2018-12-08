//! Contains modules for various application configuration

pub use self::{
    error::{Error, ErrorKind, Result},
    find::{find, find_in},
    find_context::FindContext,
    format::Format,
    io_utils::IoUtils,
    load::{load, load_in},
};

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
mod io_utils;
mod load;

// Must be after `mod find;` as it uses the `development_base_dirs` macro.
pub mod dir;
