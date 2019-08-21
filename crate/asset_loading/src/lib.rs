#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Discovers and indexes assets.
//!
//! This crate provides the logic to discover

#[cfg(test)]
#[macro_use]
extern crate hamcrest;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use crate::{
    asset_discovery::AssetDiscovery,
    asset_indexer::AssetIndexer,
    dir_traverse::DirTraverse,
    namespace_directory::NamespaceDirectory,
    namespace_discoverer::{ASSETS_DEFAULT_DIR, ASSETS_DOWNLOAD_DIR, ASSETS_TEST_DIR},
    yaml_format::YamlFormat,
};
pub(crate) use crate::{
    asset_indexing_utils::AssetIndexingUtils, map_indexer::MapIndexer,
    namespace_discoverer::NamespaceDiscoverer, object_indexer::ObjectIndexer,
};

mod asset_discovery;
mod asset_indexer;
mod asset_indexing_utils;
mod dir_traverse;
mod map_indexer;
mod namespace_directory;
mod namespace_discoverer;
mod object_indexer;
mod yaml_format;
