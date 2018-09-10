#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides built-in (compiled) assets and asset slugs.

extern crate game_model;
#[macro_use]
extern crate lazy_static;
extern crate map_model;

pub use common::NAMESPACE_BUILT_IN;
pub use map::{MAP_BLANK, MAP_BLANK_NAME, MAP_BLANK_SLUG};

mod common;
mod map;
