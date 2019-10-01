#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides functions for finding files and loading configuration.
//!
//! Please see the documentation for the [`find_in`][find_in] and [`load_in`]
//! [load_in] functions for detailed explanations.
//!
//! # Examples
//!
//! ```rust
//! use application::{dir, AppFile};
//!
//! fn main() {
//!     let path_to_resource = AppFile::find_in(dir::RESOURCES, "config.ron").unwrap();
//!
//!     println!("{:?}", path_to_resource);
//!     // "/path/to/crate/application/resources/config.ron"
//! }
//! ```
//!
//! [find_in]: resource/fn.find_in.html
//! [load_in]: resource/fn.load_in.html

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use crate::{
    app_file::AppFile, discovery_context::DiscoveryContext, find_context::FindContext,
    format::Format, io_utils::IoUtils,
};

pub mod dir;

#[cfg(test)]
#[macro_use]
mod test_support;
mod app_file;
mod discovery_context;
mod find_context;
mod format;
mod io_support;
mod io_utils;
