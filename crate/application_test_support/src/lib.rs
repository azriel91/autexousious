#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Test harness to support testing of Autexousious applications.
//!
//! This builds on top of the `amethyst_test_support` crate by providing single calls to common
//! application setups necessary to test Autexousious applications.

extern crate amethyst;
extern crate amethyst_test_support;
extern crate application;
extern crate asset_loading;
extern crate character_selection;
extern crate game_input;
extern crate game_loading;
extern crate game_model;
#[macro_use]
extern crate lazy_static;
extern crate loading;
extern crate map_loading;
extern crate map_model;
extern crate map_selection;
extern crate object_loading;
extern crate object_model;
#[cfg(test)]
extern crate strum;
pub use autexousious_application::{
    AutexousiousApplication, ASSETS_CHAR_BAT_NAME, ASSETS_CHAR_BAT_SLUG, ASSETS_MAP_FADE_NAME,
    ASSETS_MAP_FADE_SLUG,
};

mod autexousious_application;
