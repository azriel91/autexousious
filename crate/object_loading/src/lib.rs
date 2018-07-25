#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Processes object configuration into the loaded object model.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
#[cfg(not(test))]
extern crate application;
#[cfg(test)]
#[macro_use]
extern crate application;
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
#[cfg(test)]
#[macro_use]
extern crate serde_derive;
extern crate sprite_loading;
#[cfg(test)]
extern crate tempfile;

pub use object::{CharacterLoader, ObjectLoader};
pub use object_loading_bundle::ObjectLoadingBundle;

mod animation;
mod object;
mod object_loading_bundle;
