use std::collections::HashMap;
use std::fs::ReadDir;
use std::io;
use std::path::{Path, PathBuf};

use itertools::Itertools;
use object_config::ObjectType;

use config_type::ConfigType;
use index::{ConfigIndex, ConfigRecord};

/// Directory under `assets` with the default application configuration.
const DEFAULT_CONFIG_DIR: &str = "default";

/// Returns the configuration index of the `assets` directory.
pub fn index_configuration(assets_dir: &Path) -> ConfigIndex {
    // ["default", "download"]
    let conf_dirs = config_dirs(assets_dir);

    // [(Object, "default/object"), (Object, "download/object")]
    let distributed_type_config_dirs = conf_dirs
        .iter()
        .flat_map(into_type_config_dirs)
        .collect::<Vec<(ConfigType, PathBuf)>>();

    // { Object: ["default/object", "download/object"] }
    let type_config_dirs = distributed_type_config_dirs
        .into_iter()
        .into_group_map::<ConfigType, PathBuf>();

    let objects = type_config_dirs
        .get(&ConfigType::Object)
        .map_or(HashMap::new(), into_object_config_records);

    ConfigIndex { objects }
}

/// Returns the top level game configuration directories within the `assets` directory.
///
/// Currently this only contains the `default` directory. In the future it should be expanded to
/// include the directories for downloaded configuration.
///
/// # Parameters
///
/// * `assets_dir`: Path to the assets directory.
fn config_dirs(assets_dir: &Path) -> Vec<PathBuf> {
    vec![DEFAULT_CONFIG_DIR, "download"]
        .iter()
        .map(|dir_name| assets_dir.join(dir_name))
        .filter(|dir| dir.is_dir())
        .collect::<Vec<PathBuf>>()
}

/// Returns a map of configuration type to their directory.
///
/// For example:
///
/// ```text
/// {
///     ConfigType::Object: PathBuf::from("assets/objects"),
///     ConfigType::Map: PathBuf::from("assets/maps"),
/// }
/// ```
fn into_type_config_dirs(config_dir: &PathBuf) -> Vec<(ConfigType, PathBuf)> {
    ConfigType::variants()
        .into_iter()
        .map(|config_type| {
            let type_config_dir = config_dir.join(&config_type.name());
            (config_type, type_config_dir)
        })
        .filter(|&(ref _config_type, ref dir)| dir.is_dir())
        .collect::<Vec<(_, _)>>()
}

/// Returns a map of object types to the discovered configuration records.
///
/// Takes a list of object configuration directories, and indexes the configuration for each object
/// type within their respective directories. Example of an object configuration directories are:
///
/// ```rust,ignore
/// vec![
///     PathBuf::from("assets/default/object"),
///     PathBuf::from("assets/download/object"),
/// ];
/// ```
///
/// # Parameters
///
/// * `paths`: Object configuration directories to traverse.
fn into_object_config_records(paths: &Vec<PathBuf>) -> HashMap<ObjectType, Vec<ConfigRecord>> {
    ObjectType::variants()
        .into_iter()
        .filter_map(|object_type| {
            // Discover object type configuration directories
            // i.e. "assets/default/object/<object_type>"
            let object_type_dir = paths
                .iter()
                .map(|object_dir| object_dir.join(&object_type.name()))
                .filter(|path| path.is_dir())
                .collect::<Vec<PathBuf>>();

            if object_type_dir.is_empty() {
                return None;
            }

            // Loop through all of the object type configuration directories, and list their child
            // directories. Each of these should be an object configuration directory.
            let object_config_records = object_type_dir
                .iter()
                .filter_map(into_read_dir_opt)
                .map(|read_dir| {
                    read_dir
                        .filter_map(|entry| entry.ok())
                        .filter_map(|entry| {
                            let metadata = entry.metadata().ok()?;
                            if metadata.is_dir() {
                                Some(entry.path())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<PathBuf>>()
                })
                .flat_map(into_config_records)
                .collect::<Vec<ConfigRecord>>();

            Some((object_type, object_config_records))
        })
        .collect::<HashMap<ObjectType, Vec<ConfigRecord>>>()
}

fn into_config_records(paths: Vec<PathBuf>) -> Vec<ConfigRecord> {
    paths
        .into_iter()
        .map(ConfigRecord::new)
        .collect::<Vec<ConfigRecord>>()
}

/// Returns `Some(ReadDir)` if successfully reading the directory.
///
/// If it fails, an error is logged and this returns `None`.
///
/// # Parameters
///
/// * `dir`: The directory to read.
fn into_read_dir_opt(dir: &PathBuf) -> Option<ReadDir> {
    match dir.read_dir() {
        Ok(read_dir) => Some(read_dir),
        // kcov-ignore-start
        // There isn't much perceived value in automatically testing this
        Err(io_err) => match io_err.kind() {
            io::ErrorKind::NotFound => None,
            _ => {
                error!(
                    "Error occured when attempting to read directory: '{}'. Error: '{}'",
                    dir.display(),
                    &io_err
                );
                None
            }
        },
        // kcov-ignore-end
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fs;

    use object_config::ObjectType;
    use tempfile::tempdir;

    use super::{index_configuration, DEFAULT_CONFIG_DIR};
    use index::{ConfigIndex, ConfigRecord};

    #[test]
    fn empty_assets_directory_returns_empty_configuration_index() {
        let assets_dir = tempdir().unwrap();

        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            ConfigIndex::default(),
            index_configuration(&assets_dir.path())
        );
    }

    #[test]
    fn empty_default_directory_returns_empty_configuration_index() {
        let assets_dir = tempdir().unwrap();
        fs::create_dir(assets_dir.path().join(DEFAULT_CONFIG_DIR)).unwrap();

        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            ConfigIndex::default(),
            index_configuration(&assets_dir.path())
        );
    }

    #[test]
    fn empty_objects_directory_returns_empty_object_types_map() {
        let assets_dir = tempdir().unwrap();
        fs::create_dir_all(assets_dir.path().join("default/object")).unwrap();

        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            HashMap::new(),
            index_configuration(&assets_dir.path()).objects
        );
    }

    #[test]
    fn empty_object_type_directory_returns_empty_object_records() {
        let assets_dir = tempdir().unwrap();
        fs::create_dir_all(assets_dir.path().join("default/object/character")).unwrap();

        assert_eq!(
            Some(&Vec::new()),
            index_configuration(&assets_dir.path())
                .objects
                .get(&ObjectType::Character)
        );
    }

    #[test]
    fn merged_empty_object_type_directory_returns_empty_object_records() {
        let assets_dir = tempdir().unwrap();
        fs::create_dir_all(assets_dir.path().join("default/object")).unwrap();
        fs::create_dir_all(assets_dir.path().join("download/object/character")).unwrap();

        assert_eq!(
            Some(&Vec::new()),
            index_configuration(&assets_dir.path())
                .objects
                .get(&ObjectType::Character)
        );
    }

    #[test]
    fn multiple_config_dirs_are_merged() {
        let assets_dir = tempdir().unwrap();
        let path_char_a = assets_dir.path().join("default/object/character/char_a");
        let path_char_b = assets_dir.path().join("download/object/character/char_b");
        fs::create_dir_all(&path_char_a).unwrap();
        fs::create_dir_all(&path_char_b).unwrap();

        assert_eq!(
            Some(&vec![
                ConfigRecord::new(path_char_a),
                ConfigRecord::new(path_char_b),
            ]),
            index_configuration(&assets_dir.path())
                .objects
                .get(&ObjectType::Character)
        );
    }

    #[test]
    fn ignores_files_in_object_type_directory() {
        let assets_dir = tempdir().unwrap();
        let path_char_a = assets_dir.path().join("default/object/character/char_a");
        let path_char_b = assets_dir.path().join("default/object/character/char_b");
        fs::create_dir_all(&path_char_a).unwrap();
        fs::File::create(&path_char_b).unwrap();

        assert_eq!(
            Some(&vec![ConfigRecord::new(path_char_a)]),
            index_configuration(&assets_dir.path())
                .objects
                .get(&ObjectType::Character)
        );
    }
}
