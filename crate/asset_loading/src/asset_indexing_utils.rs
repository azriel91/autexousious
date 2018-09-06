use std::path::PathBuf;

use application::resource::IoUtils;
use game_model::config::{AssetRefBuilder, ConfigRecord};

/// Indexes assets within a single namespace directory.
#[derive(Debug)]
pub struct AssetIndexingUtils;

impl AssetIndexingUtils {
    /// Returns a `ConfigRecord` from the provided namespace and path.
    ///
    /// # Parameters
    ///
    /// * `namespace`: Namespace of the asset.
    /// * `path`: Path to the asset directory.
    pub fn into_config_record(namespace: String, path: PathBuf) -> Option<ConfigRecord> {
        let mapping_result = IoUtils::basename(&path)
            .map_err(|e| format!("{}", e))
            .and_then(|name| {
                AssetRefBuilder::default()
                    .namespace(namespace)
                    .name(name)
                    .build()
            }).and_then(|asset_ref| Ok(ConfigRecord::new(asset_ref, path)));

        match mapping_result {
            Ok(config_record) => Some(config_record),
            Err(e) => {
                error!("Failed to map path into config record: `{}`.", e);

                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use game_model::config::{AssetRefBuilder, ConfigRecord};

    use super::AssetIndexingUtils;

    #[test]
    fn returns_config_record_when_namespace_and_path_valid() {
        let path = PathBuf::from("my/asset");
        assert_eq!(
            Some(config_record("user1", "asset", path.clone())),
            AssetIndexingUtils::into_config_record("user1".to_string(), path)
        );
    }

    #[test]
    fn returns_none_when_namespace_invalid() {
        assert_eq!(
            None,
            AssetIndexingUtils::into_config_record(
                " invalid".to_string(),
                PathBuf::from("my/asset")
            )
        );
    }

    #[test]
    fn returns_none_when_path_invalid() {
        assert_eq!(
            None,
            AssetIndexingUtils::into_config_record("user1".to_string(), PathBuf::from("/"))
        );
    }

    fn config_record(namespace: &str, name: &str, directory: PathBuf) -> ConfigRecord {
        ConfigRecord {
            asset_ref: AssetRefBuilder::default()
                .namespace(namespace.to_string())
                .name(name.to_string())
                .build()
                .expect("Failed to build asset ref."),
            directory,
        }
    }
}
