#[cfg(test)]
mod test {
    use std::{
        io::{self, Write},
        path::Path,
    };

    use tempfile::NamedTempFile;

    use application::IoUtils;

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
