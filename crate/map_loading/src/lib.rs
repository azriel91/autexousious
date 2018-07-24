#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Processes map configuration into the loaded map model.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
#[macro_use]
extern crate derive_new;
extern crate map_model;

pub use map_loading_bundle::MapLoadingBundle;

mod map_loading_bundle;
