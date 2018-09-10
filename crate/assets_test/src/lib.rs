#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides assets for testing and asset slugs.
//!
//! **WARNING:** This crate is intended strictly for testing, and should not be used in any
//! non-`test` code. This is because it exports (and hence exposes) the path of this crate's
//! directory on the machine it is compiled on.
//!
//! For assets that should be compiled into the executable, please use the `assets_built_in` crate.

extern crate game_model;
extern crate heck;
#[macro_use]
extern crate lazy_static;
extern crate object_model;
extern crate strum;

pub use common::{ASSETS_PATH, NAMESPACE_TEST, NAMESPACE_TEST_PATH};
pub use map::{
    ASSETS_MAP_EMPTY_NAME, ASSETS_MAP_EMPTY_PATH, ASSETS_MAP_EMPTY_SLUG, ASSETS_MAP_FADE_NAME,
    ASSETS_MAP_FADE_PATH, ASSETS_MAP_FADE_SLUG,
};
pub use object::{
    ASSETS_CHAR_BAT_NAME, ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG, ASSETS_OBJECT_PATH,
};

mod common;
mod map;
mod object;
