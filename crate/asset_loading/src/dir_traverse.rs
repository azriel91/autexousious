use std::{
    fs::{DirEntry, ReadDir},
    io,
    path::{Path, PathBuf},
};

use log::{error, warn};
#[cfg(target_arch = "wasm32")]
use wasm_support_fs::DirAccess;

/// Functions to make directory traversal code more ergonomic.
#[derive(Debug)]
pub struct DirTraverse;

impl DirTraverse {
    /// Returns the child directories of the specified directory.
    ///
    /// This will traverse symlinks, and if the target path is a directory, will
    /// include it in the listing.
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
            DirAccess::child_dirs(dir)
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
    /// This also logs an error message if the entry's file type fails to be
    /// read.
    ///
    /// # Parameters
    ///
    /// * `entry`: The entry to map.
    pub fn entry_to_dir_path_buf(entry: &DirEntry) -> Option<PathBuf> {
        match entry.file_type() {
            Ok(file_type) => {
                if file_type.is_dir() || file_type.is_symlink() {
                    let path = entry.path();
                    if path.is_dir() { Some(path) } else { None }
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
