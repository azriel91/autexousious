use std::{
    fs::{DirEntry, ReadDir},
    io,
    path::{Path, PathBuf},
};

use log::{debug, error, warn};
#[cfg(target_arch = "wasm32")]
use web_sys::XmlHttpRequest;

/// Functions to make directory traversal code more ergonomic.
#[derive(Debug)]
pub struct DirTraverse;

impl DirTraverse {
    /// Returns the child directories of the specified directory.
    ///
    /// This will traverse symlinks, and if the target path is a directory, will include it in the
    /// listing.
    ///
    /// # Parameters
    ///
    /// * `dir`: Path of the directory to list.
    pub fn child_directories(dir: &Path) -> Vec<PathBuf> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self::entries(dir).map_or_else(Vec::new, |entries| {
                entries
                    .filter_map(|entry| Self::entry_to_dir_path_buf(&entry))
                    .collect::<Vec<_>>()
            })
        }

        #[cfg(target_arch = "wasm32")]
        {
            Self::lookup_child_dirs(dir)
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn lookup_child_dirs(dir: &Path) -> Vec<PathBuf> {
        let dir_str = format!("{}", dir.display());
        #[cfg(windows)]
        let dir_str = dir_str.replace('\\', '/');

        let xhr = XmlHttpRequest::new().expect("Failed to construct XmlHttpRequest");

        // Synchronous GET request. Should only be run in web worker.
        xhr.open_with_async("GET", dir_str.as_str(), false)
            .expect("XmlHttpRequest open failed");

        // We block here and wait for http fetch to complete
        xhr.send().expect("XmlHttpRequest send failed");

        // Status returns a result but according to javascript spec it should never return error.
        // Returns 0 if request was not completed.
        let status = xhr.status().expect("Failed to get XHR `status()`.");
        match status {
            200 => {
                let response = xhr
                    .response_text()
                    .expect("Failed to get XHR `response_text()`.");

                if let Some(response) = response {
                    const SEARCH_TERM: &str = r#"style="font-weight: bold;" href=""#;
                    let child_dirs = response
                        .lines()
                        .filter_map(|line| {
                            line.find(SEARCH_TERM)
                                .map(|find_index| find_index + SEARCH_TERM.len())
                                .map(|path_start_index| &line[path_start_index..])
                                .and_then(|path_to_end| {
                                    path_to_end
                                        .find('"')
                                        .map(|path_end_index| &path_to_end[..path_end_index])
                                })
                        })
                        .filter_map(|path_str| {
                            if path_str.contains(".git") {
                                None
                            } else {
                                #[cfg(windows)]
                                let path_str = path_str.replace('/', '\\');

                                Some(PathBuf::from(path_str))
                            }
                        })
                        .collect::<Vec<PathBuf>>();

                    warn!("Child directories for `{}`: {:?}", dir_str, child_dirs);

                    child_dirs
                } else {
                    Vec::new()
                }
            }
            404 => {
                debug!("{} not found, returning empty directory list.", dir_str);
                Vec::new()
            }
            _ => {
                let msg = xhr.status_text().expect("Failed to get XHR `status_text`.");
                error!("XmlHttpRequest failed with code {}. Error: {}", status, msg);

                Vec::new()
            }
        }
    }

    /// Returns the entries of the specified directory.
    ///
    /// # Parameters
    ///
    /// * `dir`: Path of the directory to list.
    pub fn entries(dir: &Path) -> Option<impl Iterator<Item = DirEntry>> {
        Self::read_dir_opt(dir).map(|read_dir| read_dir.filter_map(Result::ok))
    }

    /// Returns `Some(ReadDir)` if the directory is successfully read.
    ///
    /// If it fails, an error is logged and this returns `None`.
    ///
    /// # Parameters
    ///
    /// * `dir`: The directory to read.
    pub fn read_dir_opt(dir: &Path) -> Option<ReadDir> {
        match dir.read_dir() {
            Ok(read_dir) => Some(read_dir),
            Err(io_err) => match io_err.kind() {
                io::ErrorKind::NotFound => None,
                _ => {
                    error!(
                        "Failed to read directory: `{}`. Error: `{}`.",
                        dir.display(), // kcov-ignore
                        &io_err        // kcov-ignore
                    );
                    None
                }
            },
        }
    }

    /// Returns `Some(PathBuf)` if the entry is a directory, `None` otherwise.
    ///
    /// This also logs an error message if the entry's file type fails to be read.
    ///
    /// # Parameters
    ///
    /// * `entry`: The entry to map.
    pub fn entry_to_dir_path_buf(entry: &DirEntry) -> Option<PathBuf> {
        match entry.file_type() {
            Ok(file_type) => {
                if file_type.is_dir() || file_type.is_symlink() {
                    let path = entry.path();
                    if path.is_dir() {
                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            // kcov-ignore-start
            // Not sure how to cause a failure to automatically test this. Tried:
            //
            // * Setting the file permissions to `0o200`, `0o100`, or `0o000`.
            // * Removing the file after getting the `DirEntry`.
            Err(e) => {
                warn!("Failed to read file type: `{}`.", e);
                None
            } // kcov-ignore-end
        }
    }
}
