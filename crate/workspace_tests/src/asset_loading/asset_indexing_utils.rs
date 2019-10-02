#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use asset_model::config::{AssetRecord, AssetSlugBuilder};
    use pretty_assertions::assert_eq;

    use asset_loading::AssetIndexingUtils;

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
