use std::{
    ffi, io,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use amethyst::{utils::application_root_dir, Error};
use serde::Deserialize;

use crate::{FindContext, Format, IoUtils};

/// Functions to discover and interact with application files.
#[derive(Debug)]
pub struct AppFile(
    // Prevent instantiation.
    PhantomData<()>,
);

impl AppFile {
    /// Finds and returns the path to the configuration file.
    ///
    /// # Parameters:
    ///
    /// * `file_name`: Name of the file to search for which should be next to
    ///   the executable.
    pub fn find(file_name: &str) -> Result<PathBuf, Error> {
        Self::find_internal(application_root_dir(), file_name)
    }

    /// Visible for testing.
    #[inline]
    pub fn find_internal(
        exe_dir_result: io::Result<PathBuf>,
        file_name: &str,
    ) -> Result<PathBuf, Error> {
        Self::find_in_internal(exe_dir_result, Path::new(""), file_name)
    }

    /// Finds and returns the path to the configuration file within the given
    /// configuration directory.
    ///
    /// By default, configuration directories are assumed to be beside the
    /// current executable. This can be overridden with the
    /// `CARGO_MANIFEST_DIR` environmental variable. Setting this variable
    /// overrides the directory that is searched &mdash; this function does not
    /// fall back to the executable base directory.
    ///
    /// # Parameters:
    ///
    /// * `conf_dir`: Directory relative to the executable in which to search
    ///   for configuration.
    /// * `file_name`: Name of the file to search for.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use application::{AppDir, AppFile};
    ///
    /// // Search for '<application_dir>/resources/config.ron'.
    /// let path = match AppFile::find_in(AppDir::RESOURCES, "config.ron") {
    ///     Ok(path) => path,
    ///     Err(e) => panic!("Failed to find configuration file: {}", e),
    /// };
    ///
    /// println!("Path: {}", path.display());
    /// ```
    pub fn find_in<P: AsRef<Path> + AsRef<ffi::OsStr>>(
        conf_dir: P,
        file_name: &str,
    ) -> Result<PathBuf, Error> {
        Self::find_in_internal(application_root_dir(), conf_dir, file_name)
    }

    // kcov-ignore

    /// Visible for testing.
    #[inline]
    pub fn find_in_internal<P: AsRef<Path> + AsRef<ffi::OsStr>>(
        exe_dir_result: io::Result<PathBuf>,
        conf_dir: P,
        file_name: &str,
    ) -> Result<PathBuf, Error> {
        let exe_dir = exe_dir_result?;

        let base_dirs = vec![exe_dir];

        for base_dir in &base_dirs {
            let mut resource_path = base_dir.join(&conf_dir);
            resource_path.push(&file_name);

            #[cfg(not(target_arch = "wasm32"))]
            {
                if resource_path.exists() {
                    return Ok(resource_path);
                }
            }

            #[cfg(target_arch = "wasm32")]
            return Ok(resource_path);
        }

        let find_context = FindContext {
            base_dirs,
            conf_dir: PathBuf::from(&conf_dir),
            file_name: file_name.to_owned(),
        }; // kcov-ignore
        Err(find_context.into())
    }

    /// Loads and returns the data from the specified file.
    ///
    /// # Parameters:
    ///
    /// * `file_name`: Name of the file to search for relative to the
    ///   executable.
    /// * `format`: File format.
    pub fn load<T>(file_name: &str, format: Format) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        Self::load_internal(application_root_dir(), file_name, format)
    }

    /// Visible for testing.
    #[inline]
    pub fn load_internal<T>(
        exe_dir_result: io::Result<PathBuf>,
        file_name: &str,
        format: Format,
    ) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let file_path = Self::find_internal(exe_dir_result, file_name)?;
        Self::load_inner(file_path, format)
    }

    /// Loads and returns the data from the specified file.
    ///
    /// # Parameters:
    ///
    /// * `conf_dir`: Directory relative to the executable in which to search
    ///   for configuration.
    /// * `file_name`: Name of the file to search for relative to the
    ///   executable.
    /// * `format`: File [format].
    ///
    /// [format]: enum.Format.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Cargo.toml
    /// //
    /// // [dependencies]
    /// // serde = { version = "1.0", features = ["derive"] }
    ///
    /// use serde::Deserialize;
    ///
    /// use application::{AppDir, AppFile, Format};
    ///
    /// #[derive(Debug, Deserialize)]
    /// struct Config {
    ///     title: String,
    /// }
    ///
    /// // Search for '<application_dir>/resources/config.ron'.
    /// let config: Config = match AppFile::load_in(AppDir::RESOURCES, "config.ron", Format::Ron) {
    ///     Ok(path) => path,
    ///     Err(e) => panic!("Failed to load configuration file: {}", e),
    /// };
    ///
    /// println!("Config: {:?}", config);
    /// ```
    pub fn load_in<T, P>(conf_dir: P, file_name: &str, format: Format) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
        P: AsRef<Path> + AsRef<ffi::OsStr>,
    {
        let file_path = Self::find_in(conf_dir, file_name)?;
        Self::load_inner(file_path, format)
    }

    fn load_inner<T, P>(file_path: P, format: Format) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
        P: AsRef<Path> + AsRef<ffi::OsStr>,
    {
        let bytes = IoUtils::read_file(file_path.as_ref())?;

        Self::load_bytes(&bytes, format)
    }

    /// Loads and returns data from bytes.
    ///
    /// # Parameters:
    ///
    /// * `bytes`: Bytes of the data.
    /// * `format`: File [format].
    ///
    /// [format]: enum.Format.html
    pub fn load_bytes<T>(bytes: &[u8], format: Format) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let data = match format {
            Format::Ron => ron::de::from_bytes(&bytes)?,
            Format::Yaml => serde_yaml::from_slice(&bytes)?,
        };

        Ok(data)
    }
}
