use std::path::Path;

use log::error;

/// Extension methods to access files / directories on the application server.
pub trait PathAccessExt {
    /// Returns whether this path exists on the server.
    fn exists_on_server(self: &Self) -> bool;
}

use web_sys::XmlHttpRequest;

impl PathAccessExt for Path {
    fn exists_on_server(self: &Self) -> bool {
        let path_str = format!("{}", self.display());
        #[cfg(windows)]
        let path_str = path_str.replace('\\', '/');

        let xhr = XmlHttpRequest::new().expect("Failed to construct XmlHttpRequest");

        // Synchronous GET request. Should only be run in web worker.
        xhr.open_with_async("GET", path_str.as_str(), false)
            .expect("XmlHttpRequest open failed.");

        // We block here and wait for http fetch to complete
        xhr.send().expect("XmlHttpRequest send failed.");

        // Status returns a result but according to javascript spec it should never return error.
        // Returns 0 if request was not completed.
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
}
