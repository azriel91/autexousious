#![deny(missing_docs)]
// We do not deny missing_debug_implementations because the `lazy_static!` macro generates a
// non-debug implementation struct, and that macro is used widely throughout this crate.

//! Provides assets for testing and asset slugs.
//!
//! **WARNING:** This crate is intended strictly for testing, and should not be used in any
//! non-`test` code. This is because it exports (and hence exposes) the path of this crate's
//! directory on the machine it is compiled on.
//!
//! For assets that should be compiled into the executable, please use the `assets_built_in` crate.

pub use crate::{
    common::{ASSETS_PATH, NAMESPACE_TEST, NAMESPACE_TEST_PATH},
    map::{
        MAP_EMPTY_NAME, MAP_EMPTY_PATH, MAP_EMPTY_SLUG, MAP_FADE_NAME, MAP_FADE_PATH, MAP_FADE_SLUG,
    },
    object::{
        ASSETS_OBJECT_PATH, CHAR_BAT_NAME, CHAR_BAT_PATH, CHAR_BAT_SLUG,
        CHAR_BAT_SPRITE_BROWN_NAME, CHAR_BAT_SPRITE_GREY_NAME,
    },
};

mod common;
mod map;
mod object;
