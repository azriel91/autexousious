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
            })
            .and_then(|asset_slug| Ok(AssetRecord::new(asset_slug, path)));

        match mapping_result {
            Ok(asset_record) => Some(asset_record),
            Err(e) => {
                error!("Failed to map path into asset record: `{}`.", e);

                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use asset_model::config::{AssetRecord, AssetSlugBuilder};

    use super::AssetIndexingUtils;

    #[test]
    fn returns_asset_record_when_namespace_and_path_valid() {
        let path = PathBuf::from("my/asset");
        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            Some(asset_record("user1", "asset", path.clone())),
            AssetIndexingUtils::asset_record("user1".to_string(), path)
        );
    }

    #[test]
    fn returns_none_when_namespace_invalid() {
        assert_eq!(
            None,
            AssetIndexingUtils::asset_record(" invalid".to_string(), PathBuf::from("my/asset"))
        );
    }

    #[test]
    fn returns_none_when_path_invalid() {
        assert_eq!(
            None,
            AssetIndexingUtils::asset_record("user1".to_string(), PathBuf::from("/"))
        );
    }

    fn asset_record(namespace: &str, name: &str, path: PathBuf) -> AssetRecord {
        AssetRecord {
            asset_slug: AssetSlugBuilder::default()
                .namespace(namespace.to_string())
                .name(name.to_string())
                .build()
                .expect("Failed to build asset slug."),
            path,
        }
    }
}
