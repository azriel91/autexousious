use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::{Component, Path},
};

use log::debug;

/// One-liner functions to interact with files.
#[derive(Debug)]
pub struct IoUtils;

impl IoUtils {
    /// Returns the contents of specified file.
    ///
    /// Care must be taken to ensure the file is not large, as this does not do
    /// any file size checking.
    ///
    /// # Parameters
    ///
    /// * `file_path`: `Path` to the file to read.
    pub fn read_file(file_path: &Path) -> io::Result<Vec<u8>> {
        debug!("Reading file: {}", file_path.display());
        let mut file = BufReader::new(File::open(file_path)?);
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /// Returns the basename of the path as a String.
    ///
    /// The path must contain at least one textual segment, for example:
    ///
    /// ```rust
    /// use std::path::Path;
    ///
    /// use application::IoUtils;
    ///
    /// let path = Path::new("directory/child");
    /// let basename = IoUtils::basename(path).unwrap();
    ///
    /// assert_eq!("child", basename);
    /// ```
    pub fn basename(path: &Path) -> io::Result<String> {
        let mut components = path.components();

        // <https://doc.rust-lang.org/std/path/enum.Component.html>
        if let Some(Component::Normal(basename_os_str)) = components.next_back() {
            let basename_opt = basename_os_str.to_str();
            if let Some(basename) = basename_opt {
                Ok(basename.to_string())
            } else {
                // kcov-ignore-start
                // We can't actually construct an invalid unicode path, but just in case we hit
                // this in the wild, the code is here.
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Failed to convert basename `{}` into String. It is not valid unicode.",
                        basename_os_str.to_string_lossy()
                    ),
                ))
                // kcov-ignore-end
            } // kcov-ignore
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Failed to determine basename component of path: `{}`.",
                    path.display()
                ),
            ))
        }
    }
}
