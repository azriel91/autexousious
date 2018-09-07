use std::collections::HashMap;
use std::path::Path;

use game_model::config::AssetRecord;
use heck::SnakeCase;
use object_model::ObjectType;
use strum::IntoEnumIterator;

use {AssetIndexingUtils, DirTraverse};

/// Indexes object types' assets.
#[derive(Debug)]
pub struct ObjectIndexer;

impl ObjectIndexer {
    /// Returns `AssetRecord`s for each of the objects in the namespace.
    ///
    /// # Parameters
    ///
    /// * `namespace`: Namespace that the objects reside in.
    /// * `object_types_dir`: Directory containing all object types' assets.
    pub fn index(
        namespace: &str,
        object_types_dir: &Path,
    ) -> HashMap<ObjectType, Vec<AssetRecord>> {
        ObjectType::iter().fold(HashMap::new(), |mut objects_by_type, object_type| {
            let object_type_dir = object_types_dir.join(&object_type.to_string().to_snake_case());
            let object_dirs = DirTraverse::child_directories(&object_type_dir);

            objects_by_type.insert(
                object_type,
                object_dirs
                    .into_iter()
                    .filter_map(|object_dir| {
                        AssetIndexingUtils::into_asset_record(namespace.to_string(), object_dir)
                    }).collect::<Vec<_>>(),
            );

            objects_by_type
        })
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

    use super::ObjectIndexer;

    #[test]
    fn returns_asset_record_for_each_object() -> io::Result<()> {
        let objects_tempdir = tempdir()?;
        let objects_dir = objects_tempdir.path();

        let character_dir = objects_dir.join("character");
        let char_0_dir = character_dir.join("char_0");
        let char_1_dir = character_dir.join("char_1");
        [&character_dir, &char_0_dir, &char_1_dir]
            .iter()
            .fold(Ok(()), |result, dir| {
                result.and_then(|_| fs::create_dir(&dir))
            })?;

        let object_assets_records = ObjectIndexer::index("rara", &objects_dir);

        assert_that!(
            object_assets_records.get(&ObjectType::Character).unwrap(),
            contains(vec![
                asset_record("rara", "char_0", char_0_dir),
                asset_record("rara", "char_1", char_1_dir),
            ]).exactly()
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
