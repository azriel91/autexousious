use std::path::Path;

use asset_model::config::AssetRecord;

use crate::{AssetIndexingUtils, DirTraverse};

/// Indexes assets directly under the specified directory..
#[derive(Debug)]
pub struct FlatIndexer;

impl FlatIndexer {
    /// Returns `AssetRecords` for each of the asset_type in the namespace.
    ///
    /// # Parameters
    ///
    /// * `namespace`: Namespace that the asset_type reside in.
    /// * `asset_type_dir`: Directory containing asset_type's assets.
    pub fn index(namespace: &str, asset_type_dir: &Path) -> Vec<AssetRecord> {
        DirTraverse::child_directories(asset_type_dir)
            .into_iter()
            .filter_map(|asset_dir| {
                AssetIndexingUtils::asset_record(namespace.to_string(), asset_dir)
            })
            .collect::<Vec<_>>()
    }
}
