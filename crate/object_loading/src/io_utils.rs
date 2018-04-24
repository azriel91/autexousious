use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use error::Result;

#[derive(Debug)]
pub(crate) struct IoUtils;

impl IoUtils {
    /// Returns the contents of specified file.
    ///
    /// Care must be taken to ensure the file is not large, as this does not do any file size checking.
    ///
    /// # Parameters
    ///
    /// * `file_path`: `Path` to the file to read.
    pub(crate) fn read_file<'f>(file_path: &Path) -> Result<Vec<u8>> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod test {
    use std::io::prelude::*;
    use std::path::Path;

    use tempfile::NamedTempFile;

    use super::IoUtils;
    use error::ErrorKind;

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
            Err(e) => match *e.kind() {
                ErrorKind::Io(ref _io_error) => {}       // pass
                _ => panic!("Expected `ErrorKind::Io`"), // kcov-ignore
            },
        }
    }
}
