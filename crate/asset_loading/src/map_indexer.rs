use std::path::Path;

use asset_model::config::AssetRecord;

use crate::{AssetIndexingUtils, DirTraverse};

/// Indexes map assets.
#[derive(Debug)]
pub struct MapIndexer;

impl MapIndexer {
    /// Returns `AssetRecords` for each of the maps in the namespace.
    ///
    /// # Parameters
    ///
    /// * `namespace`: Namespace that the maps reside in.
    /// * `maps_dir`: Directory containing maps' assets.
    pub fn index(namespace: &str, maps_dir: &Path) -> Vec<AssetRecord> {
        DirTraverse::child_directories(&maps_dir)
            .into_iter()
            .filter_map(|object_dir| {
                AssetIndexingUtils::asset_record(namespace.to_string(), object_dir)
            })
            .collect::<Vec<_>>()
    }
}
