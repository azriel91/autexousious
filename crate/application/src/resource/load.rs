use std::{
    ffi,
    fs::File,
    io,
    path::{Path, PathBuf},
};

use amethyst::{utils::application_root_dir, Error};
use ron;
use serde::Deserialize;
use serde_yaml;

use crate::{find_in, resource::find_internal, Format, IoUtils};

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
    load_internal(application_root_dir(), file_name, format)
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
    let file_path = find_internal(exe_dir_result, file_name)?;
    load_inner(file_path, format)
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
/// use application::resource::{self, dir, load_in};
///
/// #[derive(Debug, Deserialize)]
/// struct Config {
///     title: String,
/// }
///
/// # fn main() {
/// // Search for '<application_dir>/resources/config.ron'.
/// let config: Config = match load_in(
///     dir::RESOURCES,
///     "config.ron",
///     resource::Format::Ron,
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
    let file_path = find_in(conf_dir, file_name)?;
    load_inner(file_path, format)
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

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use amethyst::utils::application_root_dir;
    use ron::{
        self,
        de::{ParseError, Position},
    };
    use serde::Deserialize;
    use serde_yaml;
    use tempfile::tempdir;

    use super::{load, load_in, load_internal};
    use crate::resource::{dir, test_support::setup_temp_file, FindContext, Format};

    #[test]
    fn load_in_ron_returns_resource_when_file_exists_and_parses_successfully() {
        let exe_dir = tempdir().unwrap();

        let (temp_dir, resource_path) = setup_temp_file(
            exe_dir.path(),
            dir::RESOURCES,
            "test__load_config",
            ".ron",
            Some("Data(val: 123)"),
        );
        let temp_dir = temp_dir.unwrap();

        assert_eq!(
            Data { val: 123 },
            load_in(&temp_dir.path(), "test__load_config.ron", Format::Ron,).unwrap()
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
            load_internal(
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

        let load_result = load_in::<Data, _>("", "test__load_config.ron", Format::Ron);

        if let Some(find_context) = load_result
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

        if let Some(find_context) = load::<Data>("test__load_config.ron", Format::Ron)
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
        let load_result = load_internal::<Data>(
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
            dir::RESOURCES,
            "test__load_config",
            ".yaml",
            Some("val: 123"),
        );
        let temp_dir = temp_dir.unwrap();

        assert_eq!(
            Data { val: 123 },
            load_in(&temp_dir.path(), "test__load_config.yaml", Format::Yaml,).unwrap()
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
        let load_result = load_internal::<Data>(
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
