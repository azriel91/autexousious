use std::{
    env, ffi, io,
    path::{Path, PathBuf},
};

use amethyst::Error;

use crate::resource::FindContext;

/// Returns development-time base directories as a `Vec<::std::path::Path>`.
///
/// Currently this includes the following directories:
///
/// * [`option_env!`][1]`("`[`OUT_DIR`][2]`")`
/// * [`option_env!`][1]`("`[`CARGO_MANIFEST_DIR`][2]`")`
///
/// This has to be invoked by the consumer crate as the environmental variables change based on
/// which crate invokes the macro. This cannot be a function as the environmental variables are
/// evaluated at compile time of this crate.
///
/// [1]: https://doc.rust-lang.org/std/macro.option_env.html
/// [2]: http://doc.crates.io/environment-variables.html#environment-variables-cargo-sets-for-crates
#[macro_export]
macro_rules! development_base_dirs {
    () => {{
        // If we are compiling tests, or compiling using the `debug` profile, add development
        // directories.
        //
        // Note: If you have two crates, `B` and `C`, where `B` invokes this macro, and `C` invokes
        // `B` in `#[cfg(test)]`, `B` will NOT be in `test` mode.
        let base_dirs = if cfg!(test) || cfg!(debug_assertions) {
            vec![option_env!("OUT_DIR"), option_env!("CARGO_MANIFEST_DIR")]
        } else {
            vec![]
        };
        base_dirs
            .into_iter()
            .filter_map(|dir| dir)
            .map(|dir| ::std::path::Path::new(&dir).to_owned())
            .collect()
    }};
}

/// Finds and returns the path to the configuration file.
///
/// # Parameters:
///
/// * `file_name`: Name of the file to search for which should be next to the executable.
pub fn find(file_name: &str) -> Result<PathBuf, Error> {
    find_in(Path::new(""), file_name, None)
}

/// Finds and returns the path to the configuration file within the given configuration directory.
///
/// By default, configuration directories are assumed to be beside the current executable. This can
/// be overridden with the `APP_DIR` environmental variable. Setting this variable overrides the
/// directory that is searched &mdash; this function does not fall back to the executable base
/// directory.
///
/// # Parameters:
///
/// * `conf_dir`: Directory relative to the executable in which to search for configuration.
/// * `file_name`: Name of the file to search for.
/// * `additional_base_dirs`: Additional base directories to look into. Useful at development time
///   when configuration is generated and placed in a separate output directory.
///
/// # Examples
///
/// ```rust
/// use application::{
///     development_base_dirs,
///     resource::{dir, find_in},
/// };
///
/// # fn main() {
/// // Search for '<application_dir>/resources/config.ron'.
/// let path = match find_in(
///     dir::RESOURCES,
///     "config.ron",
///     Some(development_base_dirs!()))
/// {
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
    additional_base_dirs: Option<Vec<PathBuf>>,
) -> Result<PathBuf, Error> {
    find_in_internal(
        env::current_exe(),
        conf_dir,
        file_name,
        additional_base_dirs,
    )
} // kcov-ignore

#[inline]
pub(crate) fn find_in_internal<P: AsRef<Path> + AsRef<ffi::OsStr>>(
    current_exe_result: io::Result<PathBuf>,
    conf_dir: P,
    file_name: &str,
    additional_base_dirs: Option<Vec<PathBuf>>,
) -> Result<PathBuf, Error> {
    let mut exe_dir = current_exe_result?;
    exe_dir.pop();

    // Use `APP_DIR` environment directory if set, else default to executable parent directory.
    let app_dir_env = env::var_os("APP_DIR");
    let app_dir = app_dir_env
        .as_ref()
        .map_or(exe_dir, |env_app_dir| Path::new(env_app_dir).to_path_buf());

    let mut base_dirs = vec![app_dir];

    if let Some(mut additional_dirs) = additional_base_dirs {
        base_dirs.append(&mut additional_dirs);
    }

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

    use super::{find, find_in};
    use crate::{
        development_base_dirs,
        resource::{
            dir,
            test_support::{exe_dir, setup_temp_file},
            FindContext,
        },
    };

    test_mutex!();

    test! {
        fn find_in_returns_resource_path_when_file_exists() {
            let (temp_dir, resource_path) =
                setup_temp_file(dir::RESOURCES, "test__find_config", ".ron", None);
            let temp_dir = temp_dir.unwrap();

            let expected = temp_dir.path().join("test__find_config.ron");
            assert_eq!(
                expected,
                find_in(
                    &temp_dir.path(),
                    "test__find_config.ron",
                    Some(development_base_dirs!())
                ).unwrap()
            );

            resource_path.close().unwrap();
            temp_dir.close().unwrap();
        }
    }

    test! {
        fn find_returns_resource_path_when_file_exists() {
            let (_, resource_path) =
                setup_temp_file("", "test__find_config", ".ron", None);

            assert_eq!( // kcov-ignore
                exe_dir().join("test__find_config.ron"),
                find("test__find_config.ron").unwrap()
            );

            resource_path.close().unwrap();
        }
    }

    test! {
        fn find_returns_error_when_file_does_not_exist() {
            // We don't setup_temp_file(..);

            if let Some(find_context) =
                find("test__find_config.ron")
                    .unwrap_err()
                    .as_error()
                    .downcast_ref::<Box<FindContext>>()
            {
                let base_dirs = vec![exe_dir()];
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
    }

    test! {
        fn find_in_returns_error_when_file_does_not_exist() {
            // We don't setup_temp_file(..);

            let find_result = find_in(
                "",
                "test__find_config.ron",
                None,
            );

            if let Some(find_context) = find_result
                .unwrap_err()
                .as_error()
                .downcast_ref::<Box<FindContext>>()
            {
                let base_dirs = vec![exe_dir()];
                let expected = FindContext {
                    base_dirs,
                    conf_dir: PathBuf::from(""),
                    file_name: "test__find_config.ron".to_owned(),
                }; // kcov-ignore

                assert_eq!(&Box::new(expected), find_context);
            } else {
                panic!("Expected `find_in` to return error"); // kcov-ignore
            }
        }
    }
}
