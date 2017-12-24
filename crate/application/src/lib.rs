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
//! use application::config::find_in;
//!
//! fn main() {
//!     let path_to_resource = find_in(
//!         "examples",
//!         "config.ron",
//!         Some(development_base_dirs!()),
//!     ).unwrap();
//!
//!     println!("{:?}", path_to_resource);
//!     // "/path/to/crate/application/examples/config.ron"
//! }
//! ```
//!

#[macro_use]
extern crate derive_error_chain;
extern crate error_chain;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
extern crate tempdir;
#[cfg(test)]
extern crate tempfile;

pub mod config;
