#[cfg(test)]
mod tests {
    use std::{fs, io, path::PathBuf};

    use asset_model::config::{AssetRecord, AssetSlugBuilder};
    use hamcrest::prelude::*;
    use object_type::ObjectType;
    use tempfile::tempdir;

    use asset_loading::{AssetDiscovery, ASSETS_DEFAULT_DIR, ASSETS_DOWNLOAD_DIR, ASSETS_TEST_DIR};

    #[test]
    fn returns_merged_asset_index() -> io::Result<()> {
        let assets_tempdir = tempdir()?;
        let assets_dir = assets_tempdir.path();

        let map_0_dir = assets_dir.join(
            [ASSETS_DEFAULT_DIR, "map", "map_0"]
                .iter()
                .collect::<PathBuf>(),
        );
        let map_1_dir = assets_dir.join(
            [ASSETS_TEST_DIR, "map", "map_1"]
                .iter()
                .collect::<PathBuf>(),
        );
        let char_0_dir = assets_dir.join(
            [
                ASSETS_DOWNLOAD_DIR,
                "user1",
                "object",
                "character",
                "char_0",
            ]
            .iter()
            .collect::<PathBuf>(),
        );
        [&map_0_dir, &map_1_dir, &char_0_dir]
            .iter()
            .fold(Ok(()), |result, dir| {
                result.and_then(|_| fs::create_dir_all(&dir))
            })?;

        let asset_index = AssetDiscovery::asset_index(&assets_dir);

        assert_that!(
            &asset_index.maps,
            contains(vec![
                asset_record(ASSETS_DEFAULT_DIR, "map_0", map_0_dir),
                asset_record(ASSETS_TEST_DIR, "map_1", map_1_dir),
            ])
            .exactly()
        );
        // kcov-ignore-start
        assert_that!(
            // kcov-ignore-end
            asset_index.objects.get(&ObjectType::Character).unwrap(),
            contains(vec![asset_record("user1", "char_0", char_0_dir),]).exactly()
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
