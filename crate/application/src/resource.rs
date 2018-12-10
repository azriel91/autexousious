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

pub mod dir;
mod error;
mod find;
mod find_context;
mod format;
mod io_support;
mod io_utils;
mod load;
