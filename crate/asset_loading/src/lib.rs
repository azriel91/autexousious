#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Discovers and indexes assets.
//!
//! This crate provides the logic to discover

extern crate application;
#[macro_use]
extern crate derive_new;
extern crate game_model;
#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate heck;
extern crate itertools;
#[macro_use]
extern crate log;
extern crate map_model;
extern crate object_model;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate strum;
#[cfg(test)]
extern crate tempfile;

pub use asset_discovery::AssetDiscovery;
pub use asset_indexer::AssetIndexer;
pub(crate) use asset_indexing_utils::AssetIndexingUtils;
pub use dir_traverse::DirTraverse;
pub(crate) use map_indexer::MapIndexer;
pub use namespace_directory::NamespaceDirectory;
pub(crate) use namespace_discoverer::NamespaceDiscoverer;
#[cfg(test)]
pub(crate) use namespace_discoverer::{ASSETS_DEFAULT_DIR, ASSETS_DOWNLOAD_DIR, ASSETS_TEST_DIR};
pub(crate) use object_indexer::ObjectIndexer;

mod asset_discovery;
mod asset_indexer;
mod asset_indexing_utils;
mod dir_traverse;
mod map_indexer;
mod namespace_directory;
mod namespace_discoverer;
mod object_indexer;
