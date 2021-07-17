#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Discovers and indexes assets.
//!
//! This crate provides the logic to discover assets from an `"assets"`
//! directory beside the application.
//!
//! The entry point to using this crate is `AssetDiscovery::asset_index`.

pub use crate::{
    asset_discovery::AssetDiscovery,
    asset_indexer::AssetIndexer,
    asset_indexing_utils::AssetIndexingUtils,
    dir_traverse::DirTraverse,
    flat_indexer::FlatIndexer,
    namespace_directory::NamespaceDirectory,
    namespace_discoverer::{
        NamespaceDiscoverer, ASSETS_DEFAULT_DIR, ASSETS_DOWNLOAD_DIR, ASSETS_TEST_DIR,
    },
    object_indexer::ObjectIndexer,
    yaml_format::YamlFormat,
};

mod asset_discovery;
mod asset_indexer;
mod asset_indexing_utils;
mod dir_traverse;
mod flat_indexer;
mod namespace_directory;
mod namespace_discoverer;
mod object_indexer;
mod yaml_format;
