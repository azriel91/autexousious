#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides functions for finding files and loading configuration
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

extern crate amethyst;
#[macro_use]
extern crate derive_error_chain;
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
extern crate tempdir;
#[cfg(test)]
extern crate tempfile;

pub mod resource;
