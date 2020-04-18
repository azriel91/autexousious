use std::path::{Path, PathBuf};

use log::{debug, error};

/// Accesses a directory on the server.
#[derive(Debug)]
pub struct DirAccess;

use web_sys::XmlHttpRequest;

impl DirAccess {
    /// Returns the child directories of the given directory.
    pub fn child_dirs(dir: &Path) -> Vec<PathBuf> {
        let dir_str = format!("{}", dir.display());
        #[cfg(windows)]
        let dir_str = dir_str.replace('\\', '/');

        let xhr = XmlHttpRequest::new().expect("Failed to construct XmlHttpRequest");

        // Synchronous GET request. Should only be run in web worker.
        xhr.open_with_async("GET", dir_str.as_str(), false)
            .expect("XmlHttpRequest open failed.");

        // We block here and wait for http fetch to complete
        xhr.send().expect("XmlHttpRequest send failed.");

        // Status returns a result but according to javascript spec it should never return error.
        // Returns 0 if request was not completed.
        let status = xhr.status().expect("Failed to get XHR `status()`.");
        match status {
            200 => {
                let response = xhr
                    .response_text()
                    .expect("Failed to get XHR `response_text()`.");
                let child_dirs = response.map(|response| Self::parse_child_dirs(response.as_str()));

                if let Some(child_dirs) = child_dirs {
                    debug!("Child directories for `{}`: {:?}", dir_str, child_dirs);

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

    fn parse_child_dirs(response: &str) -> Vec<PathBuf> {
        const SEARCH_TERM: &str = r#"style="font-weight: bold;" href=""#;

        response
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
            .collect::<Vec<PathBuf>>()
    }
}
