#[cfg(test)]
mod tests {
    use std::{fs, io, path::PathBuf};

    use asset_model::config::{AssetRecord, AssetSlugBuilder};
    use hamcrest::prelude::*;
    use tempfile::tempdir;

    use asset_loading::FlatIndexer;

    #[test]
    fn returns_asset_record_for_each_asset() -> io::Result<()> {
        let asset_type_tempdir = tempdir()?;
        let asset_type_dir = asset_type_tempdir.path();

        let asset_0_dir = asset_type_dir.join("asset_0");
        let asset_1_dir = asset_type_dir.join("asset_1");
        [&asset_0_dir, &asset_1_dir]
            .iter()
            .fold(Ok(()), |result, dir| {
                result.and_then(|_| fs::create_dir(&dir))
            })?;

        assert_that!(
            &FlatIndexer::index("rara", &asset_type_dir),
            contains(vec![
                asset_record("rara", "asset_0", asset_0_dir),
                asset_record("rara", "asset_1", asset_1_dir),
            ])
            .exactly()
        );

        Ok(())
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
