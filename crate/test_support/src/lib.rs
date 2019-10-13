#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//!

/// Deserializes the yaml file adjacent to the current file.
///
/// # Parameters
///
/// * `file_name`: The name of the file to load, e.g. `"my_data.yaml"`.
/// * `ty`: Type that the file deserializes into, e.g. `MyData`.
///
/// # Examples
///
/// ```rust,ignore
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Deserialize, Serialize, PartialEq)]
/// struct Data {
///     value: u32,
/// }
///
/// let data = test_support::load_yaml!("data.yaml", Data);
/// assert_eq!(Data { value: 123 }, data);
/// ```
#[macro_export]
macro_rules! load_yaml {
    ($file_name:expr, $ty:ty) => {{
        use std::path::{Path, PathBuf};

        use application::IoUtils;

        let file_to_load = {
            // Need to do this, as `file!()` returns a path relative to the repository root.
            let mut file_to_load = {
                let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
                let mut repo_dir = crate_dir.to_path_buf();
                repo_dir.pop();
                repo_dir.pop();
                repo_dir
            };

            let rs_file = Path::new(file!());
            file_to_load.push(rs_file);

            file_to_load.pop();
            file_to_load.push($file_name);
            file_to_load
        };

        let contents = IoUtils::read_file(&file_to_load).unwrap_or_else(|e| {
            panic!("Failed to read `{}`. Error: {}", file_to_load.display(), e)
        });

        serde_yaml::from_slice::<$ty>(&contents).expect(concat!(
            "Failed to load `",
            $file_name,
            "`."
        ))
    }};
}
