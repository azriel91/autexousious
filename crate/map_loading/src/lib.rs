#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes map configuration into the loaded map model.

pub use crate::{map_ascl::MapAscl, map_loading_bundle::MapLoadingBundle};

mod map_ascl;
mod map_loading_bundle;
