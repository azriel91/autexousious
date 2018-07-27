#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Provides functionality to discover and manage game configuration.
//!
//! This crate contains the types necessary to discover game configuration from the file system; it
//! does not contain the types that represent actual configuration. Those are provided by the
//! respective configuration crates.
//!
//! For example, this crate contains the [`ConfigIndex`][cfg_index] type, which stores where object
//! configuration is, but does not contain `ObjectType` or types for the various object types.
//!
//! # Examples
//!
//! ```rust
//! extern crate game_model;
//!
//! use std::path::PathBuf;
//!
//! use game_model::config;
//!
//! fn main() {
//!     let assets_dir = PathBuf::from(format!("{}/assets", env!("CARGO_MANIFEST_DIR")));
//!     let config_index = config::index_configuration(&assets_dir);
//!     println!("{:#?}", config_index);
//! }
//! ```
//!
//! [cfg_index]: config/enum.ConfigIndex.html

#[macro_use]
extern crate derive_more;
extern crate heck;
extern crate itertools;
#[macro_use]
extern crate log;
extern crate object_model;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[cfg(test)]
extern crate tempfile;

pub mod config;
