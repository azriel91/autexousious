use std::env;
use std::ffi;
use std::path::{Path, PathBuf};

use resource::error::Result;
use resource::FindContext;

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
    () => {
        vec![option_env!("OUT_DIR"), option_env!("CARGO_MANIFEST_DIR")]
            .iter()
            .filter(|dir| dir.is_some())
            .map(|dir| dir.expect("Unwrapping option"))
            .map(|dir| ::std::path::Path::new(&dir).to_owned())
            .collect()
    }
}

/// Finds and returns the path to the configuration file.
///
/// # Parameters:
///
/// * `file_name`: Name of the file to search for which should be next to the executable.
pub fn find(file_name: &str) -> Result<PathBuf> {
    find_in(Path::new(""), file_name, None)
}

/// Finds and returns the path to the configuration file within the given configuration directory.
///
/// # Parameters:
///
/// * `conf_dir`: Directory relative to the executable in which to search for configuration.
/// * `file_name`: Name of the file to search for.
/// * `additional_base_dirs`: Additional base directories to look into. Useful at development time
///     when configuration is generated and placed in a separate output directory.
pub fn find_in<P: AsRef<Path> + AsRef<ffi::OsStr>>(
    conf_dir: P,
    file_name: &str,
    additional_base_dirs: Option<Vec<PathBuf>>,
) -> Result<PathBuf> {
    let mut exe_dir = env::current_exe()?;
    exe_dir.pop();

    let mut base_dirs = vec![exe_dir];

    if let Some(mut additional_dirs) = additional_base_dirs {
        base_dirs.append(&mut additional_dirs);
    }

    if cfg!(debug_assertions) {
        base_dirs.push(development_base_dirs!());
    }

    for base_dir in &base_dirs {
        let mut conf_path = base_dir.join(&conf_dir);
        conf_path.push(&file_name);

        if conf_path.exists() {
            return Ok(conf_path);
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
    use std::env;
    use std::panic;
    use std::path::PathBuf;
    use std::sync::Mutex;

    use tempdir::TempDir;
    use tempfile::{NamedTempFile, NamedTempFileOptions};

    use resource::error::ErrorKind;
    use resource::FindContext;
    use super::{find, find_in};

    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    macro_rules! test {
        (fn $name:ident() $body:block) => {
            #[test]
            fn $name() {
                let guard = TEST_MUTEX.lock().unwrap();
                if let Err(e) = panic::catch_unwind(|| { $body }) {
                    // kcov-ignore-start
                    drop(guard);
                    panic::resume_unwind(e);
                    // kcov-ignore-end
                }
            }
        }
    }

    fn exe_dir() -> PathBuf {
        let mut exe_dir = env::current_exe().unwrap();
        exe_dir.pop();
        exe_dir
    }

    fn setup(conf_dir: Option<&str>) -> Option<(Option<TempDir>, NamedTempFile)> {
        if let Some(conf_dir) = conf_dir {
            let conf_path = PathBuf::from(conf_dir);

            // normalize relative paths to be relative to exe directory instead of working directory
            let exe_dir = exe_dir();
            let conf_parent;
            let temp_dir;

            // if the conf_path is absolute, or is the exe directory, we don't create a temp_dir
            if conf_path.is_absolute() || conf_dir == "" {
                conf_parent = exe_dir;
                temp_dir = None;
            } else {
                let tmp_dir = TempDir::new_in(exe_dir, conf_dir).unwrap();

                conf_parent = tmp_dir.path().to_owned();
                temp_dir = Some(tmp_dir);
            } // kcov-ignore

            return Some((
                temp_dir,
                NamedTempFileOptions::new()
                                    .prefix("config")
                                    .suffix(".ron")
                                    // don't include randomly generated bytes in the file name
                                    .rand_bytes(0)
                                    .create_in(conf_parent)
                                    .unwrap(),
            )); // kcov-ignore
        }

        None
    }

    test! {
        fn find_in_returns_conf_path_when_conf_file_exists() {
            let (temp_dir, conf_path) = setup(Some("resources")).unwrap();
            let temp_dir = temp_dir.unwrap();

            let expected = temp_dir.path().join("config.ron");
            assert_eq!(
                expected,
                find_in(
                    &temp_dir.path(),
                    "config.ron",
                    Some(development_base_dirs!())
                ).unwrap()
            );

            conf_path.close().unwrap();
            temp_dir.close().unwrap();
        }
    }

    test! {
        fn find_returns_conf_path_when_conf_file_exists() {
            let (_, conf_path) = setup(Some("")).unwrap();

            assert_eq!(exe_dir().join("config.ron"), find("config.ron").unwrap());

            conf_path.close().unwrap();
        }
    }

    test! {
        fn find_returns_error_when_conf_file_does_not_exist() {
            let _ = setup(None);

            if let &ErrorKind::Find(ref find_context) = find("config.ron").unwrap_err().kind() {
                let mut base_dirs = vec![exe_dir()];
                base_dirs.append(&mut development_base_dirs!());
                let expected = FindContext {
                    base_dirs,
                    conf_dir: PathBuf::from(""),
                    file_name: "config.ron".to_owned(),
                }; // kcov-ignore

                assert_eq!(&expected, find_context);
            } else {
                panic!("Expected `find` to return error"); // kcov-ignore
            }
        }
    }

}
