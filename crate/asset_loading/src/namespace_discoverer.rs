use std::path::Path;

use application::IoUtils;
use log::error;

use crate::{DirTraverse, NamespaceDirectory};

/// Directory under `assets` with test application configuration.
pub const ASSETS_TEST_DIR: &str = "test";
/// Directory under `assets` with default application configuration.
pub const ASSETS_DEFAULT_DIR: &str = "default";
/// Directory under `assets` with downloaded application configuration.
pub const ASSETS_DOWNLOAD_DIR: &str = "download";

/// Discovers namespaces in the assets directory.
#[derive(Debug)]
pub struct NamespaceDiscoverer;

impl NamespaceDiscoverer {
    /// Returns the namespace level directories within the `assets` directory.
    ///
    /// Currently this contains the following directories:
    ///
    /// * "test"
    /// * "default"
    /// * "download/*"
    ///
    /// # Parameters
    ///
    /// * `assets_dir`: Path to the assets directory.
    pub fn discover(assets_dir: &Path) -> Vec<NamespaceDirectory> {
        let dir_download = assets_dir.join(ASSETS_DOWNLOAD_DIR);
        let namespaces_downloaded = DirTraverse::child_directories(&dir_download)
            .into_iter()
            .filter_map(|directory| {
                let basename = IoUtils::basename(&directory);
                match basename {
                    Ok(namespace) => Some((namespace, directory)),
                    // kcov-ignore-start
                    // This case would require an invalid unicode path to be created on the file
                    // system, which is unnecessarily difficult to do.
                    Err(e) => {
                        error!("Failed to read namespace directory. Error: `{}`", e);
                        None
                    } // kcov-ignore-end
                }
            });

        vec![ASSETS_TEST_DIR.to_string(), ASSETS_DEFAULT_DIR.to_string()]
            .into_iter()
            .map(|namespace| {
                let path = assets_dir.join(&namespace);
                (namespace, path)
            })
            .filter(|(_namespace, dir)| dir.is_dir())
            .chain(namespaces_downloaded)
            .map(|(namespace, path)| NamespaceDirectory { namespace, path })
            .collect::<Vec<_>>()
    }
}
