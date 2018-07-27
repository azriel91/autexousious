#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Processes map configuration into the loaded map model.

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
#[macro_use]
extern crate log;
extern crate map_model;
extern crate sprite_loading;

pub use map_loader::MapLoader;
pub use map_loading_bundle::MapLoadingBundle;

mod map_loader;
mod map_loading_bundle;
