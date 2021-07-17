//! Constants for resource directories.

use std::{
    io,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use amethyst::{utils::application_root_dir, Error};

use crate::AppFile;
#[cfg(not(target_arch = "wasm32"))]
use crate::DiscoveryContext;

/// Functions to discover and interact with application files.
#[derive(Debug)]
pub struct AppDir(
    // Prevent instantiation.
    PhantomData<()>,
);

impl AppDir {
    // Note to self:
    //
    // I know in code we use the singular form of the noun, whereas the directory
    // names are plural. This is in line with Amethyst's convention of resource
    // directories.

    /// `assets` directory name.
    pub const ASSETS: &'static str = "assets";
    /// `resources` directory name.
    pub const RESOURCES: &'static str = "resources";

    /// Returns an absolute path to the current exe's assets directory.
    ///
    /// # Errors
    ///
    /// Returns a [`Error`][res_err] with error kind
    /// [`ErrorKind::Discovery`][dir_disc] when the following scenarios
    /// occur:
    ///
    /// * Unable to retrieve current executable path.
    /// * Unable to retrieve current executable parent.
    ///
    /// [res_err]: resource/dir/struct.Error.html
    /// [dir_disc]: resource/dir/enum.ErrorKind.html#variant.Discovery
    pub fn assets() -> Result<PathBuf, Error> {
        Self::dir_internal(application_root_dir(), Self::ASSETS)
    }

    /// Returns an absolute path to the current exe's resources directory.
    ///
    /// # Errors
    ///
    /// Returns a [`Error`][res_err] with error kind
    /// [`ErrorKind::Discovery`][dir_disc] when the following scenarios
    /// occur:
    ///
    /// * Unable to retrieve current executable path.
    /// * Unable to retrieve current executable parent.
    ///
    /// [res_err]: resource/dir/struct.Error.html
    /// [dir_disc]: resource/dir/enum.ErrorKind.html#variant.Discovery
    pub fn resources() -> Result<PathBuf, Error> {
        Self::dir_internal(application_root_dir(), Self::RESOURCES)
    }

    /// Visible for testing.
    #[inline]
    pub fn dir_internal(
        current_exe_result: io::Result<PathBuf>,
        dir_name: &'static str,
    ) -> Result<PathBuf, Error> {
        let dir = AppFile::find_in_internal(current_exe_result, Path::new(""), dir_name)?;

        // Canonicalize path to handle symlinks.
        #[cfg(not(target_arch = "wasm32"))]
        match dir.canonicalize() {
            Ok(dir) => {
                if dir.is_dir() {
                    Ok(dir)
                } else {
                    Err(DiscoveryContext::new(dir_name, "Path is not a directory.", None).into())
                }
            }
            // kcov-ignore-start
            // This case is quite unlikely -- it *could* happen, for example, if the underlying
            // directory is deleted or renamed after `find_in_internal` has found the directory.
            Err(io_error) => Err(DiscoveryContext::new(
                dir_name,
                "Failed to canonicalize path. Please ensure directory exists and can be accessed.",
                Some(io_error),
            )
            .into()),
            // kcov-ignore-end
        }

        #[cfg(target_arch = "wasm32")]
        Ok(dir)
    }
}
