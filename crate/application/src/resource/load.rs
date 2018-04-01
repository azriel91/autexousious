use std::ffi;
use std::fs::File;
use std::path::{Path, PathBuf};

use ron::de::from_reader;
use serde::Deserialize;

use resource::error::Result;
use resource::find::{find, find_in};
use resource::format::Format;

/// Loads and returns the data from the specified file.
///
/// # Parameters:
///
/// * `file_name`: Name of the file to search for relative to the executable.
/// * `format`: File format.
pub fn load<T>(file_name: &str, format: &Format) -> Result<T>
where
    for<'de> T: Deserialize<'de>,
{
    let file_path = find(file_name)?;
    load_internal(file_path, format)
}

/// Loads and returns the data from the specified file.
///
/// # Parameters:
///
/// * `conf_dir`: Directory relative to the executable in which to search for configuration.
/// * `file_name`: Name of the file to search for relative to the executable.
/// * `format`: File format.
/// * `additional_base_dirs`: Additional base directories to look into. Useful at development time
///     when configuration is generated and placed in a separate output directory.
pub fn load_in<T, P>(
    conf_dir: P,
    file_name: &str,
    format: &Format,
    additional_base_dirs: Option<Vec<PathBuf>>,
) -> Result<T>
where
    for<'de> T: Deserialize<'de>,
    P: AsRef<Path> + AsRef<ffi::OsStr>,
{
    let file_path = find_in(conf_dir, file_name, additional_base_dirs)?;
    load_internal(file_path, format)
}

fn load_internal<T, P>(file_path: P, format: &Format) -> Result<T>
where
    for<'de> T: Deserialize<'de>,
    P: AsRef<Path> + AsRef<ffi::OsStr>,
{
    let file_reader = File::open(file_path)?;
    match *format {
        Format::Ron => Ok(from_reader(file_reader)?),
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use ron;
    use ron::de::ParseError;

    use super::{load, load_in};
    use resource::dir;
    use resource::error::ErrorKind;
    use resource::test_support::{exe_dir, setup_temp_file};
    use resource::{FindContext, Format};

    test_mutex!();

    test! {
        fn load_in_returns_resource_when_file_exists_and_parses_successfully() {
            let (temp_dir, resource_path) = setup_temp_file(
                dir::RESOURCES,
                "test__load_config",
                ".ron",
                Some("Data(123)"),
            );
            let temp_dir = temp_dir.unwrap();

            assert_eq!(
                Data(123),
                load_in(
                    &temp_dir.path(),
                    "test__load_config.ron",
                    &Format::Ron,
                    Some(development_base_dirs!())
                ).unwrap()
            );

            resource_path.close().unwrap();
            temp_dir.close().unwrap();
        }
    }

    test! {
        fn load_returns_resource_when_file_exists_and_parses_successfully() {
            let (_, resource_path) =
                setup_temp_file("", "test__load_config", ".ron", Some("Data(123)"));

            assert_eq!(
                Data(123),
                load("test__load_config.ron", &Format::Ron).unwrap()
            );

            resource_path.close().unwrap();
        }
    }

    test! {
        fn load_in_returns_error_when_file_does_not_exist() {
            // We don't setup_temp_file(..);

            let load_result = load_in::<Data, _>(
                "",
                "test__load_config.ron",
                &Format::Ron,
                None,
            );

            if let &ErrorKind::Find(ref find_context) = load_result.unwrap_err().kind() {
                let mut base_dirs = vec![exe_dir()];
                if cfg!(debug_assertions) {
                    base_dirs.push(development_base_dirs!());
                }
                let expected = FindContext {
                    base_dirs,
                    conf_dir: PathBuf::from(""),
                    file_name: "test__load_config.ron".to_owned(),
                }; // kcov-ignore

                assert_eq!(&expected, find_context);
            } else {
                panic!("Expected `load_in` to return error"); // kcov-ignore
            }
        }
    }

    test! {
        fn load_returns_error_when_file_does_not_exist() {
            // We don't setup_temp_file(..);

            if let &ErrorKind::Find(ref find_context) =
                load::<Data>("test__load_config.ron", &Format::Ron).unwrap_err().kind()
            {
                let mut base_dirs = vec![exe_dir()];
                if cfg!(debug_assertions) {
                    base_dirs.push(development_base_dirs!());
                }
                let expected = FindContext {
                    base_dirs,
                    conf_dir: PathBuf::from(""),
                    file_name: "test__load_config.ron".to_owned(),
                }; // kcov-ignore

                assert_eq!(&expected, find_context);
            } else {
                panic!("Expected `load` to return error"); // kcov-ignore
            }
        }
    }

    test! {
        fn load_returns_error_when_file_fails_to_parse() {
            let (_, resource_path) = setup_temp_file(
                "",
                "test__load_config",
                ".ron",
                Some("I'm parsable. Unparsable."),
            );

            let load_result = load::<Data>("test__load_config.ron", &Format::Ron);
            resource_path.close().unwrap();

            // We cannot use `assert_eq!` because `ron::parse::Position` is private
            match load_result.expect_err("Expected parse failure.").kind() {
                &ErrorKind::RonDeserialization(ron::de::Error::Parser(
                    ParseError::ExpectedStruct,
                    ..
                )) => (),
                _ => panic!("Expected RonDeserialization error"), // kcov-ignore
            };
        }
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Data(i32);
}
