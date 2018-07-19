#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Provides functions for finding files and loading configuration.
//!
//! Please see the documentation for the [`resource::find_in`][find_in] and [`resource::load_in`]
//! [load_in] functions for detailed explanations.
//!
//! # Examples
//!
//! ```rust
//! #[macro_use] // for the development_base_dirs!() macro
//! extern crate application;
//!
//! use application::resource::dir;
//! use application::resource::find_in;
//!
//! fn main() {
//!     let path_to_resource = find_in(
//!         dir::RESOURCES,
//!         "config.ron",
//!         Some(development_base_dirs!()),
//!     ).unwrap();
//!
//!     println!("{:?}", path_to_resource);
//!     // "/path/to/crate/application/resources/config.ron"
//! }
//! ```
//!
//! [find_in]: resource/fn.find_in.html
//! [load_in]: resource/fn.load_in.html

extern crate amethyst;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_error_chain;
#[macro_use]
extern crate derive_more;
extern crate error_chain;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate ron;
#[cfg(test)]
#[macro_use]
extern crate serde;
#[cfg(not(test))]
extern crate serde;
#[cfg(test)]
extern crate tempfile;

pub mod resource;
