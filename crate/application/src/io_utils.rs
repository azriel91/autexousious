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
    /// Care must be taken to ensure the file is not large, as this does not do any file size
    /// checking.
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
    /// # fn main() {
    /// let path = Path::new("directory/child");
    /// let basename = IoUtils::basename(path).unwrap();
    ///
    /// assert_eq!("child", basename);
    /// # }
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
                // We can't actually construct an invalid unicode path, but just in case we hit this
                // in the wild, the code is here.
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

#[cfg(test)]
mod test {
    use std::{
        io::{self, Write},
        path::Path,
    };

    use tempfile::NamedTempFile;

    use super::IoUtils;

    #[test]
    fn reads_file_to_bytes() {
        let mut named_temp_file = NamedTempFile::new().expect("Failed to create file.");
        {
            let file = named_temp_file.as_file_mut();
            write!(file, "abcde").expect("Failed to write to file.");
        }

        let bytes = IoUtils::read_file(named_temp_file.path()).expect("Failed to read file.");

        assert_eq!(vec![0x61, 0x62, 0x63, 0x64, 0x65], bytes);
    }

    #[test]
    fn returns_crate_error_when_file_fails_to_open() {
        match IoUtils::read_file(Path::new("")) {
            Ok(_) => panic!("Expected failure to read invalid file path."), // kcov-ignore
            Err(e) => assert_eq!(io::ErrorKind::NotFound, e.kind()),
        }
    }

    #[test]
    fn basename_returns_basename_when_valid() {
        let path = Path::new("directory/child");
        let basename = IoUtils::basename(path).expect("Expected basename to be valid");

        assert_eq!("child", basename);
    }

    #[test]
    fn basename_returns_error_when_no_text_segment() {
        let path = Path::new("/");
        let error = IoUtils::basename(&path).expect_err("Expected basename to fail.");

        match error.kind() {
            io::ErrorKind::Other => {
                assert_eq!(
                    "Failed to determine basename component of path: `/`.",
                    format!("{}", error)
                );
            }
            _ => panic!("Expected `ErrorKind::Msg`."), // kcov-ignore
        }
    }
}
