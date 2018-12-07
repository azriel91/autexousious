use std::path::Path;

use game_model::config::AssetIndex;

use crate::{AssetIndexer, NamespaceDiscoverer};

/// Discovers assets across multiple namespaces.
#[derive(Debug)]
pub struct AssetDiscovery;

impl AssetDiscovery {
    /// Returns the asset index of the `assets` directory.
    ///
    /// # Parameters
    ///
    /// * `assets_dir`: Path to the assets directory to index.
    pub fn asset_index(assets_dir: &Path) -> AssetIndex {
        let namespace_directories = NamespaceDiscoverer::discover(assets_dir);
        namespace_directories.iter().map(AssetIndexer::index).fold(
            AssetIndex::default(),
            |mut combined, asset_index| {
                combined.maps.extend(asset_index.maps);
                combined.objects.extend(asset_index.objects);

                combined
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::path::PathBuf;

    use game_model::config::{AssetRecord, AssetSlugBuilder};
    use hamcrest::prelude::*;
    use object_model::ObjectType;
    use tempfile::tempdir;

    use super::AssetDiscovery;
    use crate::{ASSETS_DEFAULT_DIR, ASSETS_DOWNLOAD_DIR, ASSETS_TEST_DIR};

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
