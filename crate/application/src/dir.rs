//! Constants for resource directories.

use std::{
    io,
    path::{Path, PathBuf},
};

use amethyst::{utils::application_root_dir, Error};

use crate::{AppFile, DiscoveryContext};

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
/// # Errors
///
/// Returns a [`Error`][res_err] with error kind [`ErrorKind::Discovery`][dir_disc]
/// when the following scenarios occur:
///
/// * Unable to retrieve current executable path.
/// * Unable to retrieve current executable parent.
///
/// [res_err]: resource/dir/struct.Error.html
/// [dir_disc]: resource/dir/enum.ErrorKind.html#variant.Discovery
pub fn assets_dir() -> Result<PathBuf, Error> {
    assets_dir_internal(application_root_dir())
}

#[inline]
fn assets_dir_internal(current_exe_result: io::Result<PathBuf>) -> Result<PathBuf, Error> {
    let dir = AppFile::find_in_internal(current_exe_result, Path::new(""), ASSETS)?;

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
        )
        .into()),
        // kcov-ignore-end
    }
}

#[cfg(test)]
mod test {
    #[cfg(unix)]
    use std::os::unix;
    #[cfg(windows)]
    use std::os::windows;
    use std::{
        fs, io,
        path::{Path, PathBuf},
    };

    use amethyst::Error;
    use tempfile::tempdir;

    use super::{assets_dir_internal, ASSETS};
    use crate::{DiscoveryContext, FindContext};

    // kcov-ignore-start
    fn assert_dir_discovery_error(
        // kcov-ignore-end
        expected_context: DiscoveryContext,
        assets_dir_result: Result<PathBuf, Error>,
    ) {
        let error = assets_dir_result.unwrap_err();
        if let Some(discovery_context) = error.as_error().downcast_ref::<Box<DiscoveryContext>>() {
            assert_eq!(&Box::new(expected_context), discovery_context);
        } else {
            // kcov-ignore-start
            panic!(
                "Expected `DiscoveryContext` error but was `{:?}`",
                error.as_error()
            );
            // kcov-ignore-end
        }
    }

    // kcov-ignore-start
    fn assert_discovery_resource_find_error(
        // kcov-ignore-end
        expected_find_context: FindContext,
        assets_dir_result: Result<PathBuf, Error>,
    ) {
        let expected_err = Error::new(expected_find_context.clone());
        let error = match assets_dir_result {
            // kcov-ignore-start
            Ok(ref _path) => panic!(
                "Expected error `{:?}` but was `{:?}`",
                expected_err, &assets_dir_result
            ),
            // kcov-ignore-end
            Err(e) => e,
        }; // kcov-ignore

        if let Some(find_context) = error.as_error().downcast_ref::<Box<FindContext>>() {
            assert_eq!(Box::new(expected_find_context), *find_context);
        } else {
            // kcov-ignore-start
            panic!(
                "Expected `FindContext` error but was `{:?}`",
                error.as_error()
            );
            // kcov-ignore-end
        }
    }

    // kcov-ignore-start
    fn assert_discovery_resource_io_error(
        // kcov-ignore-end
        expected_io_error: io::Error,
        assets_dir: Result<PathBuf, Error>,
    ) {
        let expected_io_error_kind = expected_io_error.kind().clone();
        let expected_err = Error::new(expected_io_error);
        let error = match assets_dir {
            // kcov-ignore-start
            Ok(ref _path) => panic!(
                "Expected error `{:?}` but was `{:?}`",
                expected_err, &assets_dir
            ),
            // kcov-ignore-end
            Err(e) => e,
        }; // kcov-ignore

        if let Some(io_error) = error.as_error().downcast_ref::<Box<io::Error>>() {
            assert_eq!(expected_io_error_kind, io_error.kind());
        } else {
            panic!(
                "Expected `io::Error` error but was `{:?}`",
                error.as_error()
            ); // kcov-ignore
        }
    }

    #[test]
    fn assets_dir_returns_assets_dir_path_beside_current_executable() {
        let exe_dir = tempdir().unwrap();
        let assets_path = exe_dir.path().join(ASSETS);
        let _assets_dir = fs::create_dir(&assets_path).unwrap();
        let assets_dir = assets_dir_internal(Ok(exe_dir.into_path()));

        // `error-chain` generated `Error` doesn't implement `PartialEq`, so we have to manually
        // compare
        let expected: Result<PathBuf, Error> = Ok(assets_path.canonicalize().unwrap());
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
        let assets_dir = assets_dir_internal(Err(io::Error::new(io::ErrorKind::Other, "oh no!")));

        assert_discovery_resource_io_error(
            io::Error::new(io::ErrorKind::Other, "oh no!"),
            assets_dir,
        );
    }

    #[test]
    fn assets_dir_returns_contextual_error_when_assets_dir_does_not_exist() {
        let exe_dir = tempdir().unwrap();

        let assets_dir = assets_dir_internal(Ok(exe_dir.path().to_path_buf()));

        let expected_find_context = FindContext {
            base_dirs: vec![exe_dir.into_path()],
            conf_dir: Path::new("").to_path_buf(),
            file_name: ASSETS.to_string(),
        }; // kcov-ignore
        assert_discovery_resource_find_error(expected_find_context, assets_dir);
    }

    #[test]
    fn assets_dir_returns_assets_dir_path_when_path_is_symlink_to_directory() {
        let exe_dir = tempdir().unwrap();
        let _assets_dir = fs::create_dir(exe_dir.path().join("my_assets")).unwrap();

        #[cfg(unix)]
        unix::fs::symlink(
            exe_dir.path().join("my_assets"),
            exe_dir.path().join(ASSETS),
        )
        .expect("Failed to create symlink for test.");

        #[cfg(windows)]
        {
            windows::fs::symlink_dir(
                exe_dir.path().join("my_assets"),
                exe_dir.path().join(ASSETS),
            )
            .expect("Failed to create symlink for test.");
        }

        let expected: Result<PathBuf, Error> =
            Ok(exe_dir.path().join("my_assets").canonicalize().unwrap());
        let assets_dir = assets_dir_internal(Ok(exe_dir.into_path()));
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
        let assets_file = fs::File::create(exe_dir.path().join("my_assets")).unwrap();
        drop(assets_file); // close the file

        #[cfg(unix)]
        unix::fs::symlink(
            exe_dir.path().join("my_assets"),
            exe_dir.path().join(ASSETS),
        )
        .expect("Failed to create symlink for test.");

        #[cfg(windows)]
        {
            windows::fs::symlink_file(
                exe_dir.path().join("my_assets"),
                exe_dir.path().join(ASSETS),
            )
            .expect("Failed to create symlink for test.");
        }

        let assets_dir = assets_dir_internal(Ok(exe_dir.into_path()));

        let expected_discovery_context =
            DiscoveryContext::new(ASSETS, "Path is not a directory.", None);
        assert_dir_discovery_error(expected_discovery_context, assets_dir);
    } // kcov-ignore

    #[test]
    fn assets_dir_returns_contextual_error_when_assets_symlink_points_to_non_existent_path() {
        let exe_dir = tempdir().unwrap();

        #[cfg(unix)]
        unix::fs::symlink(
            exe_dir.path().join("non_existent_assets"),
            exe_dir.path().join(ASSETS),
        )
        .expect("Failed to create symlink for test.");

        #[cfg(windows)]
        {
            windows::fs::symlink_file(
                exe_dir.path().join("non_existent_assets"),
                exe_dir.path().join(ASSETS),
            )
            .expect("Failed to create symlink for test.");
        }

        let assets_dir = assets_dir_internal(Ok(exe_dir.path().to_path_buf()));
        let base_dir = exe_dir.into_path();

        let expected_find_context = FindContext {
            base_dirs: vec![base_dir],
            conf_dir: Path::new("").to_path_buf(),
            file_name: ASSETS.to_string(),
        }; // kcov-ignore
        assert_discovery_resource_find_error(expected_find_context, assets_dir);
    }
}
