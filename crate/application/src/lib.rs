#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides functions for finding files and loading configuration.
//!
//! Please see the documentation for the [`resource::find_in`][find_in] and [`resource::load_in`]
//! [load_in] functions for detailed explanations.
//!
//! # Examples
//!
//! ```rust
//! use application::resource::{dir, find_in};
//!
//! fn main() {
//!     let path_to_resource = find_in(dir::RESOURCES, "config.ron").unwrap();
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

pub use crate::resource::{find, find_in, load, load_in, FindContext, Format, IoUtils};

pub mod resource;
