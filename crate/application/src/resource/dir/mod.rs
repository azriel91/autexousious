//! Constants for resource directories.

mod discovery_context;
mod error;

pub use self::discovery_context::DiscoveryContext;
pub use self::error::{Error, ErrorKind, Result};

use std::env;
use std::io;
use std::path::{Path, PathBuf};

use resource::find::find_in_internal;

// Note to self:
//
// I know in code we use the singular form of the noun, whereas the directory names are plural.
// This is in line with Amethyst's convention of resource directories.

/// `assets` directory name.
pub const ASSETS: &str = "assets";
/// `resources` directory name.
pub const RESOURCES: &str = "resources";

/// Returns an absolute path to the current exe's assets directory.
///
/// # Parameters
///
/// * `additional_base_dirs`: Additional directories to search.
///
///     This is exposed primarily for applications to pass in `Some(development_base_dirs!())` so
///     that the binaries may also search for artifacts in the crate directory.
///
/// # Errors
///
/// Returns a [`resource::Error`][res_err] with error kind [`ErrorKind::Discovery`][dir_disc]
/// when the following scenarios occur:
///
/// * Unable to retrieve current executable path.
/// * Unable to retrieve current executable parent.
///
/// [res_err]: resource/dir/struct.Error.html
/// [dir_disc]: resource/dir/enum.ErrorKind.html#variant.Discovery
pub fn assets_dir(additional_base_dirs: Option<Vec<PathBuf>>) -> Result<PathBuf> {
    assets_dir_internal(env::current_exe(), additional_base_dirs)
}

#[inline]
fn assets_dir_internal(
    current_exe_result: io::Result<PathBuf>,
    additional_base_dirs: Option<Vec<PathBuf>>,
) -> Result<PathBuf> {
    let dir = find_in_internal(
        current_exe_result,
        Path::new(""),
        ASSETS,
        additional_base_dirs,
    )?;

    // Canonicalize path to handle symlinks.
    match dir.canonicalize() {
        Ok(dir) => {
            if dir.is_dir() {
                Ok(dir)
            } else {
                Err(DiscoveryContext::new(ASSETS, "Path is not a directory.", None).into())
            }
        }
        // kcov-ignore-start
        // This case is quite unlikely -- it *could* happen, for example, if the underlying
        // directory is deleted or renamed after `find_in_internal` has found the directory.
        Err(io_error) => Err(DiscoveryContext::new(
            ASSETS,
            "Failed to canonicalize path. Please ensure directory exists and can be accessed.",
            Some(io_error),
        ).into()),
        // kcov-ignore-end
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io;
    #[cfg(unix)]
    use std::os::unix;
    #[cfg(windows)]
    use std::os::windows;
    use std::path::{Path, PathBuf};

    use tempfile::tempdir;

    use super::{assets_dir_internal, ASSETS};
    use resource;
    use resource::FindContext;
    use resource::dir::{DiscoveryContext, Error, ErrorKind};

    fn assert_dir_discovery_error(
        expected_context: DiscoveryContext,
        assets_dir: Result<PathBuf, Error>,
    ) {
        match *assets_dir.unwrap_err().kind() {
            ErrorKind::Discovery(ref discovery_context) => {
                assert_eq!(expected_context, *discovery_context);
            }
            // kcov-ignore-start
            ref kind @ _ => {
                panic!(
                    "Expected error with kind `{:?}` but was `{:?}`",
                    ErrorKind::Discovery(expected_context),
                    kind
                );
            } // kcov-ignore-end
        }
    }

    fn assert_discovery_resource_find_error(
        expected_find_context: FindContext,
        assets_dir: Result<PathBuf, Error>,
    ) {
        let expected: Result<PathBuf, Error> = Err(Error::from(resource::Error::from(
            expected_find_context.clone(),
        )));
        let error = match assets_dir {
            // kcov-ignore-start
            Ok(ref _path) => panic!(
                "Expected error `{:?}` but was `{:?}`",
                expected, &assets_dir
            ),
            // kcov-ignore-end
            Err(e) => e,
        }; // kcov-ignore
        let expected_err = expected.unwrap_err();
        let expected_kind = &expected_err.kind();
        let resource_error_kind = match *error.kind() {
            ErrorKind::Resource(ref resource_error_kind) => resource_error_kind,
            // kcov-ignore-start
            ref kind @ _ => {
                panic!(
                    "Expected error kind: `{:?}` but was `{:?}`",
                    expected_kind, kind
                );
            } // kcov-ignore-end
        };
        match *resource_error_kind {
            resource::ErrorKind::Find(ref find_context) => {
                assert_eq!(&expected_find_context, find_context)
            }
            // kcov-ignore-start
            _ => panic!(
                "Expected `{:?}` but was `{:?}`",
                expected_kind, resource_error_kind
            ),
        }; // kcov-ignore-end
    }

    fn assert_discovery_resource_io_error(
        expected_io_error: io::Error,
        assets_dir: Result<PathBuf, Error>,
    ) {
        let expected_io_error_kind = expected_io_error.kind().clone();
        let expected: Result<PathBuf, Error> =
            Err(Error::from(resource::Error::from(expected_io_error)));
        let error = match assets_dir {
            // kcov-ignore-start
            Ok(ref _path) => panic!(
                "Expected error `{:?}` but was `{:?}`",
                expected, &assets_dir
            ),
            // kcov-ignore-end
            Err(e) => e,
        }; // kcov-ignore
        let expected_err = expected.unwrap_err();
        let expected_kind = &expected_err.kind();
        let resource_error_kind = match *error.kind() {
            ErrorKind::Resource(ref resource_error_kind) => resource_error_kind,
            // kcov-ignore-start
            ref kind @ _ => {
                panic!(
                    "Expected error kind: `{:?}` but was `{:?}`",
                    expected_kind, kind
                );
            } // kcov-ignore-end
        };
        match *resource_error_kind {
            resource::ErrorKind::Io(ref io_error) => {
                assert_eq!(expected_io_error_kind, io_error.kind())
            }
            // kcov-ignore-start
            _ => panic!(
                "Expected `{:?}` but was `{:?}`",
                expected_kind, resource_error_kind
            ),
        }; // kcov-ignore-end
    }

    #[test]
    fn assets_dir_returns_assets_dir_path_beside_current_executable() {
        let exe_dir = tempdir().unwrap();
        let exe_path = exe_dir.path().join("current_exe");
        let _assets_dir = fs::create_dir(exe_dir.path().join(ASSETS)).unwrap();
        let assets_dir = assets_dir_internal(Ok(exe_path), None);

        // `error-chain` generated `Error` doesn't implement `PartialEq`, so we have to manually
        // compare
        let expected: Result<PathBuf, Error> = Ok(exe_dir.path().join(ASSETS));
        assert!(
            assets_dir.is_ok(),
            "Expected assets_dir to return {:?}, but was {:?}",
            expected,
            assets_dir
        );
        assert_eq!(expected.unwrap(), assets_dir.unwrap());
    }

    #[test]
    fn assets_dir_returns_contextual_error_when_failing_to_get_current_exe_path() {
        let assets_dir =
            assets_dir_internal(Err(io::Error::new(io::ErrorKind::Other, "oh no!")), None);

        assert_discovery_resource_io_error(
            io::Error::new(io::ErrorKind::Other, "oh no!"),
            assets_dir,
        );
    }

    #[test]
    fn assets_dir_returns_contextual_error_when_assets_dir_does_not_exist() {
        let exe_dir = tempdir().unwrap();
        let exe_path = exe_dir.path().join("current_exe");

        let assets_dir = assets_dir_internal(Ok(exe_path.clone()), None);

        let expected_find_context = FindContext {
            base_dirs: vec![exe_dir.path().to_path_buf()],
            conf_dir: Path::new("").to_path_buf(),
            file_name: ASSETS.to_string(),
        }; // kcov-ignore
        assert_discovery_resource_find_error(expected_find_context, assets_dir);
    }

    #[test]
    fn assets_dir_returns_assets_dir_path_when_path_is_symlink_to_directory() {
        let exe_dir = tempdir().unwrap();
        let exe_path = exe_dir.path().join("current_exe");
        let _assets_dir = fs::create_dir(exe_dir.path().join("my_assets")).unwrap();

        #[cfg(unix)]
        unix::fs::symlink(
            exe_dir.path().join("my_assets"),
            exe_dir.path().join(ASSETS),
        ).expect("Failed to create symlink for test.");

        #[cfg(windows)]
        {
            windows::fs::symlink_dir(
                exe_dir.path().join("my_assets"),
                exe_dir.path().join(ASSETS),
            ).expect("Failed to create symlink for test.");
        }

        let assets_dir = assets_dir_internal(Ok(exe_path.clone()), None);
        let expected: Result<PathBuf, Error> = Ok(exe_dir.path().join("my_assets"));
        assert!(
            assets_dir.is_ok(),
            "Expected assets_dir to return {:?}, but was {:?}",
            expected,
            assets_dir
        );
        assert_eq!(expected.unwrap(), assets_dir.unwrap());
    }

    #[test]
    fn assets_dir_returns_contextual_error_when_assets_dir_points_to_non_directory() {
        let exe_dir = tempdir().unwrap();
        let exe_path = exe_dir.path().join("current_exe");
        let assets_file = fs::File::create(exe_dir.path().join("my_assets")).unwrap();
        drop(assets_file); // close the file

        #[cfg(unix)]
        unix::fs::symlink(
            exe_dir.path().join("my_assets"),
            exe_dir.path().join(ASSETS),
        ).expect("Failed to create symlink for test.");

        #[cfg(windows)]
        {
            windows::fs::symlink_file(
                exe_dir.path().join("my_assets"),
                exe_dir.path().join(ASSETS),
            ).expect("Failed to create symlink for test.");
        }

        let assets_dir = assets_dir_internal(Ok(exe_path.clone()), None);

        let expected_discovery_context =
            DiscoveryContext::new(ASSETS, "Path is not a directory.", None);
        assert_dir_discovery_error(expected_discovery_context, assets_dir);
    }

    #[test]
    fn assets_dir_returns_contextual_error_when_assets_symlink_points_to_non_existent_path() {
        let exe_dir = tempdir().unwrap();
        let exe_path = exe_dir.path().join("current_exe");

        #[cfg(unix)]
        unix::fs::symlink(
            exe_dir.path().join("non_existent_assets"),
            exe_dir.path().join(ASSETS),
        ).expect("Failed to create symlink for test.");

        #[cfg(windows)]
        {
            windows::fs::symlink_file(
                exe_dir.path().join("non_existent_assets"),
                exe_dir.path().join(ASSETS),
            ).expect("Failed to create symlink for test.");
        }

        let assets_dir = assets_dir_internal(Ok(exe_path.clone()), None);

        let expected_find_context = FindContext {
            base_dirs: vec![exe_dir.path().to_path_buf()],
            conf_dir: Path::new("").to_path_buf(),
            file_name: ASSETS.to_string(),
        }; // kcov-ignore
        assert_discovery_resource_find_error(expected_find_context, assets_dir);
    }
}
