#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Processes map configuration into the loaded map model.

#[macro_use]
extern crate log;

pub use crate::{map_loader::MapLoader, map_loading_bundle::MapLoadingBundle};

mod map_loader;
mod map_loading_bundle;
