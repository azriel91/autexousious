#![deny(missing_docs)]
// We do not deny missing_debug_implementations because the `lazy_static!` macro generates a
// non-debug implementation struct.

//! Provides built-in (compiled) assets and asset slugs.

pub use crate::{
    common::NAMESPACE_BUILT_IN,
    map::{MAP_BLANK_NAME, MAP_BLANK_SLUG, MAP_DEFINITION_BLANK},
};

mod common;
mod map;
