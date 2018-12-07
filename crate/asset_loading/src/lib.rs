#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Discovers and indexes assets.
//!
//! This crate provides the logic to discover


#[macro_use]
extern crate derive_new;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;


#[macro_use]
extern crate log;


#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;



pub use crate::asset_discovery::AssetDiscovery;
pub use crate::asset_indexer::AssetIndexer;
pub(crate) use crate::asset_indexing_utils::AssetIndexingUtils;
pub use crate::dir_traverse::DirTraverse;
pub(crate) use crate::map_indexer::MapIndexer;
pub use crate::namespace_directory::NamespaceDirectory;
pub(crate) use crate::namespace_discoverer::NamespaceDiscoverer;
pub use crate::namespace_discoverer::{ASSETS_DEFAULT_DIR, ASSETS_DOWNLOAD_DIR, ASSETS_TEST_DIR};
pub(crate) use crate::object_indexer::ObjectIndexer;

mod asset_discovery;
mod asset_indexer;
mod asset_indexing_utils;
mod dir_traverse;
mod map_indexer;
mod namespace_directory;
mod namespace_discoverer;
mod object_indexer;
