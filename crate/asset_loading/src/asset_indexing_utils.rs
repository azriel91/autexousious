use std::path::PathBuf;

use application::IoUtils;
use asset_model::config::{AssetRecord, AssetSlugBuilder};
use log::error;

/// Utility functions to make it easier to manage asset indexing.
#[derive(Debug)]
pub struct AssetIndexingUtils;

impl AssetIndexingUtils {
    /// Returns an `AssetRecord` from the provided namespace and path.
    ///
    /// # Parameters
    ///
    /// * `namespace`: Namespace of the asset.
    /// * `path`: Path to the asset directory.
    pub fn asset_record(namespace: String, path: PathBuf) -> Option<AssetRecord> {
        let mapping_result = IoUtils::basename(&path)
            .map_err(|e| format!("{}", e))
            .and_then(|name| {
                AssetSlugBuilder::default()
                    .namespace(namespace)
                    .name(name)
                    .build()
                    .map_err(|e| e.to_string())
            })
            .map(|asset_slug| AssetRecord::new(asset_slug, path));

        match mapping_result {
            Ok(asset_record) => Some(asset_record),
            Err(e) => {
                error!("Failed to map path into asset record: `{}`.", e);

                None
            }
        }
    }
}
