use game_model::config::{ConfigIndex, ConfigType};
use heck::SnakeCase;
use strum::IntoEnumIterator;

use {MapIndexer, NamespaceDirectory, ObjectIndexer};

/// Indexes assets within a single namespace directory.
#[derive(Debug)]
pub struct AssetIndexer;

impl AssetIndexer {
    /// Returns a configuration index from a single namespace.
    ///
    /// # Parameters
    ///
    /// * `namespace_dir`: Namespace directory to index.
    pub fn index(namespace_dir: &NamespaceDirectory) -> ConfigIndex {
        ConfigType::iter().fold(ConfigIndex::default(), |mut config_index, config_type| {
            let config_type_dir = namespace_dir
                .path
                .join(&config_type.to_string().to_snake_case());

            match config_type {
                ConfigType::Object => {
                    config_index.objects =
                        ObjectIndexer::index(&namespace_dir.namespace, &config_type_dir)
                }
                ConfigType::Map => {
                    config_index.maps =
                        MapIndexer::index(&namespace_dir.namespace, &config_type_dir)
                }
            };

            config_index
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::path::PathBuf;

    use game_model::config::{AssetRefBuilder, ConfigRecord};
    use hamcrest::prelude::*;
    use object_model::ObjectType;
    use tempfile::tempdir;

    use super::AssetIndexer;
    use NamespaceDirectory;

    #[test]
    fn returns_config_index_of_maps_and_objects() -> io::Result<()> {
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

        let config_index = AssetIndexer::index(&NamespaceDirectory::new(
            "rara".to_string(),
            namespace_dir.to_path_buf(),
        ));

        assert_that!(
            &config_index.maps,
            contains(vec![
                config_record("rara", "map_0", map_0_dir),
                config_record("rara", "map_1", map_1_dir),
            ]).exactly()
        );
        assert_that!(
            config_index.objects.get(&ObjectType::Character).unwrap(),
            contains(vec![
                config_record("rara", "char_0", char_0_dir),
                config_record("rara", "char_1", char_1_dir),
            ]).exactly()
        );

        Ok(())
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
