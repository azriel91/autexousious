use asset_model::config::{AssetIndex, AssetTypeVariants};
use heck::SnakeCase;
use strum::IntoEnumIterator;

use crate::{MapIndexer, NamespaceDirectory, ObjectIndexer};

/// Indexes assets within a single namespace directory.
#[derive(Debug)]
pub struct AssetIndexer;

impl AssetIndexer {
    /// Returns an asset index for a single namespace.
    ///
    /// # Parameters
    ///
    /// * `namespace_dir`: Namespace directory to index.
    pub fn index(namespace_dir: &NamespaceDirectory) -> AssetIndex {
        AssetTypeVariants::iter().fold(AssetIndex::default(), |mut asset_index, asset_type| {
            let asset_type_dir = namespace_dir
                .path
                .join(&asset_type.to_string().to_snake_case());

            match asset_type {
                AssetTypeVariants::Object => {
                    asset_index.objects =
                        ObjectIndexer::index(&namespace_dir.namespace, &asset_type_dir)
                }
                AssetTypeVariants::Map => {
                    asset_index.maps = MapIndexer::index(&namespace_dir.namespace, &asset_type_dir)
                }
            };

            asset_index
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io, path::PathBuf};

    use asset_model::config::{AssetRecord, AssetSlugBuilder};
    use hamcrest::prelude::*;
    use object_type::ObjectType;
    use tempfile::tempdir;

    use super::AssetIndexer;
    use crate::NamespaceDirectory;

    #[test]
    fn returns_asset_index_of_maps_and_objects() -> io::Result<()> {
        let namespace_tempdir = tempdir()?;
        let namespace_dir = namespace_tempdir.path();

        let maps_dir = namespace_dir.join("map");
        let map_0_dir = maps_dir.join("map_0");
        let map_1_dir = maps_dir.join("map_1");
        let objects_dir = namespace_dir.join("object");
        let character_dir = objects_dir.join("character");
        let char_0_dir = character_dir.join("char_0");
        let char_1_dir = character_dir.join("char_1");
        [
            &maps_dir,
            &map_0_dir,
            &map_1_dir,
            &objects_dir,
            &character_dir,
            &char_0_dir,
            &char_1_dir,
        ]
        .iter()
        .fold(Ok(()), |result, dir| {
            result.and_then(|_| fs::create_dir(&dir))
        })?;

        let asset_index = AssetIndexer::index(&NamespaceDirectory::new(
            "rara".to_string(),
            namespace_dir.to_path_buf(),
        ));

        assert_that!(
            &asset_index.maps,
            contains(vec![
                asset_record("rara", "map_0", map_0_dir),
                asset_record("rara", "map_1", map_1_dir),
            ])
            .exactly()
        );
        // kcov-ignore-start
        assert_that!(
            // kcov-ignore-end
            asset_index.objects.get(&ObjectType::Character).unwrap(),
            contains(vec![
                asset_record("rara", "char_0", char_0_dir),
                asset_record("rara", "char_1", char_1_dir),
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
