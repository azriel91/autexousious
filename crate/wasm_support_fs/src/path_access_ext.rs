use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use log::error;

// Cache whether a path exists on the server.
lazy_static! {
    static ref PATH_EXISTS_CACHE: Arc<Mutex<HashMap<PathBuf, bool>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

/// Extension methods to access files / directories on the application server.
pub trait PathAccessExt {
    /// Returns whether this path exists on the server.
    fn exists_on_server(&self) -> bool;
}

use web_sys::XmlHttpRequest;

impl PathAccessExt for Path {
    fn exists_on_server(&self) -> bool {
        match PATH_EXISTS_CACHE.lock() {
            Ok(mut path_exists_cache) => *path_exists_cache
                .entry(self.to_path_buf())
                .or_insert_with(|| lookup_path_on_server(self)),
            Err(e) => {
                error!("Failed to lock `PATH_EXISTS_CACHE`: {}", e);
                false
            }
        }
    }
}

fn lookup_path_on_server(path: &Path) -> bool {
    let path_str = format!("{}", path.display());
    #[cfg(windows)]
    let path_str = path_str.replace("\\", "/");

    let xhr = XmlHttpRequest::new().expect("Failed to construct XmlHttpRequest");

    // Synchronous GET request. Should only be run in web worker.
    xhr.open_with_async("GET", path_str.as_str(), false)
        .expect("XmlHttpRequest open failed.");

    // We block here and wait for http fetch to complete
    xhr.send().expect("XmlHttpRequest send failed.");

    // Status returns a result but according to javascript spec it should never
    // return error. Returns 0 if request was not completed.
    let status = xhr.status().expect("Failed to get XHR `status()`.");
    match status {
        200 => true,
        404 => false,
        _ => {
            let msg = xhr.status_text().expect("Failed to get XHR `status_text`.");
            error!("XmlHttpRequest failed with code {}. Error: {}", status, msg);

            false
        }
    }
}
