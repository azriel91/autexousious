use std::path::{Path, PathBuf};
use std::ffi;
use std::fs::File;

use ron::de::from_reader;
use serde::Deserialize;

use resource::find::{find, find_in};
use resource::format::Format;
use resource::error::Result;

/// Loads and returns the data from the specified file.
///
/// # Parameters:
///
/// * `file_name`: Name of the file to search for relative to the executable.
/// * `format`: File format.
pub fn load<T>(file_name: &str, format: Format) -> Result<T>
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
    format: Format,
    additional_base_dirs: Option<Vec<PathBuf>>,
) -> Result<T>
where
    for<'de> T: Deserialize<'de>,
    P: AsRef<Path> + AsRef<ffi::OsStr>,
{
    let file_path = find_in(conf_dir, file_name, additional_base_dirs)?;
    load_internal(file_path, format)
}

fn load_internal<T, P>(file_path: P, format: Format) -> Result<T>
where
    for<'de> T: Deserialize<'de>,
    P: AsRef<Path> + AsRef<ffi::OsStr>,
{
    let file_reader = File::open(file_path)?;
    match format {
        Format::Ron => Ok(from_reader(file_reader)?),
    }
}
