#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Processes object configuration into the loaded object model.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
extern crate application;
#[cfg(test)]
extern crate assets_test;
#[macro_use]
extern crate derive_new;
extern crate game_model;
#[macro_use]
extern crate log;
extern crate object_model;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
extern crate serde;
extern crate sprite_loading;
#[cfg(test)]
extern crate strum;
#[cfg(test)]
extern crate tempfile;

pub use object::{CharacterLoader, ObjectLoader};
pub use object_loading_bundle::ObjectLoadingBundle;

mod object;
mod object_loading_bundle;
