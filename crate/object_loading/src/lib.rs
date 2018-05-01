#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Processes object configuration into the loaded object model.

extern crate amethyst;
#[cfg(test)]
#[macro_use]
extern crate application;
#[macro_use]
extern crate derive_error_chain;
#[macro_use]
extern crate derive_new;
extern crate error_chain;
extern crate game_model;
#[macro_use]
extern crate log;
extern crate object_model;
#[cfg(test)]
extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate tempfile;
extern crate toml;

pub use error::{Error, ErrorKind};
pub(crate) use io_utils::IoUtils;
pub use object_loader::ObjectLoader;

mod animation;
mod error;
mod io_utils;
mod object_loader;
mod sprite;
