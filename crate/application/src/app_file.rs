use std::{
    ffi,
    fs::File,
    io,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use amethyst::{utils::application_root_dir, Error};
use ron;
use serde::Deserialize;
use serde_yaml;

use crate::{FindContext, Format, IoUtils};

/// Functions to discover and interact with application files.
#[derive(Debug)]
pub struct AppFile(
    // prevent instantiation
    PhantomData<()>,
);

impl AppFile {
    /// Finds and returns the path to the configuration file.
    ///
    /// # Parameters:
    ///
    /// * `file_name`: Name of the file to search for which should be next to the executable.
    pub fn find(file_name: &str) -> Result<PathBuf, Error> {
        Self::find_internal(application_root_dir(), file_name)
    }

    #[inline]
    pub(crate) fn find_internal(
        exe_dir_result: io::Result<PathBuf>,
        file_name: &str,
    ) -> Result<PathBuf, Error> {
        Self::find_in_internal(exe_dir_result, Path::new(""), file_name)
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
    /// use application::{AppDir, AppFile};
    ///
    /// # fn main() {
    /// // Search for '<application_dir>/resources/config.ron'.
    /// let path = match AppFile::find_in(
    ///     AppDir::RESOURCES,
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
        Self::find_in_internal(application_root_dir(), conf_dir, file_name)
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

    /// Loads and returns the data from the specified file.
    ///
    /// # Parameters:
    ///
    /// * `file_name`: Name of the file to search for relative to the executable.
    /// * `format`: File format.
    pub fn load<T>(file_name: &str, format: Format) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        Self::load_internal(application_root_dir(), file_name, format)
    }

    #[inline]
    fn load_internal<T>(
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
    /// * `conf_dir`: Directory relative to the executable in which to search for configuration.
    /// * `file_name`: Name of the file to search for relative to the executable.
    /// * `format`: File [format][format].
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
    /// # fn main() {
    /// // Search for '<application_dir>/resources/config.ron'.
    /// let config: Config = match AppFile::load_in(
    ///     AppDir::RESOURCES,
    ///     "config.ron",
    ///     Format::Ron,
    /// ) {
    ///     Ok(path) => path,
    ///     Err(e) => panic!("Failed to load configuration file: {}", e),
    /// };
    ///
    /// println!("Config: {:?}", config);
    /// # }
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
        match format {
            Format::Ron => {
                let file_reader = File::open(file_path)?;
                Ok(ron::de::from_reader(file_reader)?)
            }
            Format::Yaml => {
                let yaml_contents = IoUtils::read_file(file_path.as_ref())?;
                Ok(serde_yaml::from_slice(&yaml_contents)?)
            }
        }
    }
}

#[cfg(test)]
mod test {
    mod find {
        use std::path::PathBuf;

        use amethyst::utils::application_root_dir;
        use tempfile::tempdir;

        use crate::{test_support::setup_temp_file, AppDir, AppFile, FindContext};

        #[test]
        fn find_in_returns_resource_path_when_file_exists() {
            let exe_dir = tempdir().unwrap();

            let (temp_dir, resource_path) = setup_temp_file(
                exe_dir.path(),
                AppDir::RESOURCES,
                "test__find_config",
                ".ron",
                None,
            );
            let temp_dir = temp_dir.unwrap();

            let expected = temp_dir.path().join("test__find_config.ron");
            assert_eq!(
                expected,
                AppFile::find_in_internal(
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
                AppFile::find_internal(Ok(exe_dir.into_path()), "test__find_config.ron").unwrap()
            );

            resource_path.close().unwrap();
        }

        #[test]
        fn find_returns_error_when_file_does_not_exist() {
            // We don't setup_temp_file(..);

            if let Some(find_context) = AppFile::find("test__find_config.ron")
                .unwrap_err()
                .as_error()
                .downcast_ref::<Box<FindContext>>()
            {
                let base_dirs = vec![application_root_dir()
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

            let find_result = AppFile::find_in_internal(
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

    mod load {
        use std::path::PathBuf;

        use amethyst::utils::application_root_dir;
        use ron::{
            self,
            de::{ParseError, Position},
        };
        use serde::Deserialize;
        use serde_yaml;
        use tempfile::tempdir;

        use crate::{test_support::setup_temp_file, AppDir, AppFile, FindContext, Format};

        #[test]
        fn load_in_ron_returns_resource_when_file_exists_and_parses_successfully() {
            let exe_dir = tempdir().unwrap();

            let (temp_dir, resource_path) = setup_temp_file(
                exe_dir.path(),
                AppDir::RESOURCES,
                "test__load_config",
                ".ron",
                Some("Data(val: 123)"),
            );
            let temp_dir = temp_dir.unwrap();

            assert_eq!(
                Data { val: 123 },
                AppFile::load_in(&temp_dir.path(), "test__load_config.ron", Format::Ron,).unwrap()
            );

            resource_path.close().unwrap();
            temp_dir.close().unwrap();
        }

        #[test]
        fn load_ron_returns_resource_when_file_exists_and_parses_successfully() {
            let exe_dir = tempdir().unwrap();

            let (_, resource_path) = setup_temp_file(
                exe_dir.path(),
                "",
                "test__load_config",
                ".ron",
                Some("Data(val: 123)"),
            );

            assert_eq!(
                Data { val: 123 },
                AppFile::load_internal(
                    Ok(exe_dir.into_path()),
                    "test__load_config.ron",
                    Format::Ron
                )
                .unwrap()
            );

            resource_path.close().unwrap();
        }

        #[test]
        fn load_in_ron_returns_error_when_file_does_not_exist() {
            // We don't setup_temp_file(..);

            let load_result = AppFile::load_in::<Data, _>("", "test__load_config.ron", Format::Ron);

            if let Some(find_context) = load_result
                .unwrap_err()
                .as_error()
                .downcast_ref::<Box<FindContext>>()
            {
                let base_dirs = vec![application_root_dir()
                    .expect("Failed to determine application root directory.")];
                let expected = FindContext {
                    base_dirs,
                    conf_dir: PathBuf::from(""),
                    file_name: "test__load_config.ron".to_owned(),
                }; // kcov-ignore

                assert_eq!(&Box::new(expected), find_context);
            } else {
                panic!("Expected `load_in` to return error"); // kcov-ignore
            }
        }

        #[test]
        fn load_ron_returns_error_when_file_does_not_exist() {
            // We don't setup_temp_file(..);

            if let Some(find_context) = AppFile::load::<Data>("test__load_config.ron", Format::Ron)
                .unwrap_err()
                .as_error()
                .downcast_ref::<Box<FindContext>>()
            {
                let base_dirs = vec![application_root_dir()
                    .expect("Failed to determine application root directory.")];
                let expected = FindContext {
                    base_dirs,
                    conf_dir: PathBuf::from(""),
                    file_name: "test__load_config.ron".to_owned(),
                }; // kcov-ignore

                assert_eq!(&Box::new(expected), find_context);
            } else {
                panic!("Expected `load` to return error"); // kcov-ignore
            }
        }

        #[test]
        fn load_ron_returns_error_when_file_fails_to_parse() {
            let exe_dir = tempdir().unwrap();

            let (_, resource_path) = setup_temp_file(
                exe_dir.path(),
                "",
                "test__load_config",
                ".ron",
                Some("I'm parsable. Unparsable."),
            );
            let load_result = AppFile::load_internal::<Data>(
                Ok(exe_dir.into_path()),
                "test__load_config.ron",
                Format::Ron,
            );
            resource_path.close().unwrap();

            // We cannot use `assert_eq!` because `ron::parse::Position` is private
            if let Some(boxed_error) = load_result
                .expect_err("Expected parse failure.")
                .as_error()
                .downcast_ref::<Box<ron::de::Error>>()
            {
                assert_eq!(
                    &Box::new(ron::de::Error::Parser(
                        ParseError::ExpectedStruct,
                        Position { col: 1, line: 1 }
                    )),
                    boxed_error
                )
            } else {
                panic!("Expected `ron::de::Error`."); // kcov-ignore
            }
        }

        #[test]
        fn load_in_yaml_returns_resource_when_file_exists_and_parses_successfully() {
            let exe_dir = tempdir().unwrap();

            let (temp_dir, resource_path) = setup_temp_file(
                exe_dir.path(),
                AppDir::RESOURCES,
                "test__load_config",
                ".yaml",
                Some("val: 123"),
            );
            let temp_dir = temp_dir.unwrap();

            assert_eq!(
                Data { val: 123 },
                AppFile::load_in(&temp_dir.path(), "test__load_config.yaml", Format::Yaml,)
                    .unwrap()
            );

            resource_path.close().unwrap();
            temp_dir.close().unwrap();
        }

        #[test]
        fn load_yaml_returns_error_when_file_fails_to_parse() {
            let exe_dir = tempdir().unwrap();

            let (_, resource_path) = setup_temp_file(
                exe_dir.path(),
                "",
                "test__load_config",
                ".yaml",
                Some("I'm parsable. Unparsable."),
            );
            let load_result = AppFile::load_internal::<Data>(
                Ok(exe_dir.into_path()),
                "test__load_config.yaml",
                Format::Yaml,
            );
            resource_path.close().unwrap();

            let panic_message = format!("Expected `serde_yaml::Error`. {:?}", &load_result);
            if let Some(_yaml_error) = load_result
                .expect_err("Expected parse failure.")
                .as_error()
                .downcast_ref::<Box<serde_yaml::Error>>()
            {
                // pass
            } else {
                panic!("{}", panic_message); // kcov-ignore
            }
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct Data {
            val: i32,
        }
    }
}
