#[cfg(test)]
mod test {
    use std::{
        io::Write,
        path::{Path, PathBuf},
    };

    use tempfile::{Builder, NamedTempFile, TempDir};

    mod find {
        use std::path::PathBuf;

        use amethyst::utils::application_root_dir;
        use tempfile::tempdir;

        use super::setup_temp_file;
        use application::{AppDir, AppFile, FindContext};

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
            de::{ErrorCode, Position},
        };
        use serde::Deserialize;
        use serde_yaml;
        use tempfile::tempdir;

        use super::setup_temp_file;
        use application::{AppDir, AppFile, FindContext, Format};

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
                    &Box::new(ron::de::Error {
                        code: ErrorCode::ExpectedStruct,
                        position: Position { col: 1, line: 1 }
                    }),
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

    /// Creates a temporary resource file in a directory for tests.
    ///
    /// # Parameters
    ///
    /// * `resource_dir`: Parent directory of the file. Either absolute, or relative to the executable.
    /// * `file_prefix`: File stem, such as "display_config" in "display_config.ron".
    /// * `file_suffix`: File extension including the ".", such as ".ron" in "display_config.ron".
    /// * `contents`: String to write into the file.
    fn setup_temp_file(
        exe_dir: &Path,
        resource_dir: &str,
        file_prefix: &str,
        file_suffix: &str,
        contents: Option<&str>,
    ) -> (Option<TempDir>, NamedTempFile) {
        let conf_path = PathBuf::from(resource_dir);

        // normalize relative paths to be relative to exe directory instead of working directory
        let conf_parent;
        let temp_dir;

        // if the conf_path is absolute, or is the exe directory, we don't create a temp_dir
        if conf_path.is_absolute() || resource_dir == "" {
            conf_parent = exe_dir.to_path_buf();
            temp_dir = None;
        } else {
            let tmp_dir = Builder::new()
                .prefix(resource_dir)
                .tempdir_in(exe_dir)
                .unwrap();

            conf_parent = tmp_dir.path().to_owned();
            temp_dir = Some(tmp_dir);
        } // kcov-ignore

        let mut temp_file = Builder::new()
            .prefix(file_prefix)
            .suffix(file_suffix)
            // don't include randomly generated bytes in the file name
            .rand_bytes(0)
            .tempfile_in(conf_parent)
            .unwrap();

        if let Some(contents) = contents {
            write!(temp_file, "{}", contents).expect("Failed to write contents to temp_file");
        }

        (temp_dir, temp_file)
    }
}
