use std::{
    ffi, io,
    path::{Path, PathBuf},
};

use amethyst::{utils::application_root_dir, Error};

use crate::resource::FindContext;

/// Finds and returns the path to the configuration file.
///
/// # Parameters:
///
/// * `file_name`: Name of the file to search for which should be next to the executable.
pub fn find(file_name: &str) -> Result<PathBuf, Error> {
    find_internal(application_root_dir(), file_name)
}

#[inline]
pub(crate) fn find_internal(
    exe_dir_result: io::Result<PathBuf>,
    file_name: &str,
) -> Result<PathBuf, Error> {
    find_in_internal(exe_dir_result, Path::new(""), file_name)
}

/// Finds and returns the path to the configuration file within the given configuration directory.
///
/// By default, configuration directories are assumed to be beside the current executable. This can
/// be overridden with the `CARGO_MANIFEST_DIR` environmental variable. Setting this variable
/// overrides the directory that is searched &mdash; this function does not fall back to the
/// executable base directory.
///
/// # Parameters:
///
/// * `conf_dir`: Directory relative to the executable in which to search for configuration.
/// * `file_name`: Name of the file to search for.
///
/// # Examples
///
/// ```rust
/// use application::resource::{dir, find_in};
///
/// # fn main() {
/// // Search for '<application_dir>/resources/config.ron'.
/// let path = match find_in(
///     dir::RESOURCES,
///     "config.ron",
/// ) {
///     Ok(path) => path,
///     Err(e) => panic!("Failed to find configuration file: {}", e),
/// };
///
/// println!("Path: {}", path.display());
/// # }
/// ```
pub fn find_in<P: AsRef<Path> + AsRef<ffi::OsStr>>(
    conf_dir: P,
    file_name: &str,
) -> Result<PathBuf, Error> {
    find_in_internal(application_root_dir(), conf_dir, file_name)
} // kcov-ignore

#[inline]
pub(crate) fn find_in_internal<P: AsRef<Path> + AsRef<ffi::OsStr>>(
    exe_dir_result: io::Result<PathBuf>,
    conf_dir: P,
    file_name: &str,
) -> Result<PathBuf, Error> {
    let exe_dir = exe_dir_result?;

    let base_dirs = vec![exe_dir];

    for base_dir in &base_dirs {
        let mut resource_path = base_dir.join(&conf_dir);
        resource_path.push(&file_name);

        if resource_path.exists() {
            return Ok(resource_path);
        }
    }

    let find_context = FindContext {
        base_dirs,
        conf_dir: PathBuf::from(&conf_dir),
        file_name: file_name.to_owned(),
    }; // kcov-ignore
    Err(find_context.into())
}

/// The tests in here rely on file system state, which can cause failures when one test creates a
/// temporary file, and another test expects an Error when the file does not exist (but it does).
///
/// This is mentioned in the following issues:
///
/// * https://github.com/rust-lang/rust/issues/33519
/// * https://github.com/rust-lang/rust/pull/42684#issuecomment-314224230
/// * https://github.com/rust-lang/rust/issues/43155
///
/// We use a static mutex to ensure these tests are run serially. The code is taken from the third
/// link above.
#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use amethyst::utils::application_root_dir;
    use tempfile::tempdir;

    use super::{find, find_in_internal, find_internal};
    use crate::resource::{dir, test_support::setup_temp_file, FindContext};

    #[test]
    fn find_in_returns_resource_path_when_file_exists() {
        let exe_dir = tempdir().unwrap();

        let (temp_dir, resource_path) = setup_temp_file(
            exe_dir.path(),
            dir::RESOURCES,
            "test__find_config",
            ".ron",
            None,
        );
        let temp_dir = temp_dir.unwrap();

        let expected = temp_dir.path().join("test__find_config.ron");
        assert_eq!(
            expected,
            find_in_internal(
                Ok(exe_dir.into_path()),
                &temp_dir.path(),
                "test__find_config.ron",
            )
            .unwrap()
        );

        resource_path.close().unwrap();
        temp_dir.close().unwrap();
    }

    #[test]
    fn find_returns_resource_path_when_file_exists() {
        let exe_dir = tempdir().unwrap();

        let (_, resource_path) =
            setup_temp_file(exe_dir.path(), "", "test__find_config", ".ron", None);

        assert_eq!(
            exe_dir.path().join("test__find_config.ron"),
            find_internal(Ok(exe_dir.into_path()), "test__find_config.ron").unwrap()
        );

        resource_path.close().unwrap();
    }

    #[test]
    fn find_returns_error_when_file_does_not_exist() {
        // We don't setup_temp_file(..);

        if let Some(find_context) = find("test__find_config.ron")
            .unwrap_err()
            .as_error()
            .downcast_ref::<Box<FindContext>>()
        {
            let base_dirs =
                vec![application_root_dir()
                    .expect("Failed to determine application root directory.")];
            let expected = FindContext {
                base_dirs,
                conf_dir: PathBuf::from(""),
                file_name: "test__find_config.ron".to_owned(),
            }; // kcov-ignore

            assert_eq!(&Box::new(expected), find_context);
        } else {
            panic!("Expected `find` to return error"); // kcov-ignore
        }
    }

    #[test]
    fn find_in_returns_error_when_file_does_not_exist() {
        let exe_dir = tempdir().unwrap();

        // We don't setup_temp_file(..);

        let find_result = find_in_internal(
            Ok(exe_dir.path().to_path_buf()),
            "",
            "test__find_config.ron",
        );

        if let Some(find_context) = find_result
            .unwrap_err()
            .as_error()
            .downcast_ref::<Box<FindContext>>()
        {
            let base_dirs = vec![exe_dir.into_path()];
            let expected = FindContext {
                base_dirs,
                conf_dir: PathBuf::from(""),
                file_name: "test__find_config.ron".to_owned(),
            }; // kcov-ignore

            assert_eq!(&Box::new(expected), find_context);
        } else {
            panic!("Expected `find_in_internal` to return error"); // kcov-ignore
        }
    }
}
