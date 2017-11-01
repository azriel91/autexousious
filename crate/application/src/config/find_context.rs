use std::fmt;
use std::path::PathBuf;

/// Context of a find operation
///
/// Useful for debugging failures to find application configuration.
#[derive(Debug, PartialEq, Eq)]
pub struct FindContext {
    /// Base directories searched during the find
    ///
    /// This includes:
    ///
    /// * Base directory of the executable
    /// * During development, directory specified by `OUT_DIR`
    /// * During development, directory specified by `CARGO_MANIFEST_DIR`
    pub base_dirs: Vec<PathBuf>,
    /// Directory relative to the base directories to look into
    pub conf_dir: PathBuf,
    /// Name of the file searched for
    pub file_name: String,
}

impl fmt::Display for FindContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to find '{}' under any of the following directories:\n",
            self.conf_dir.join(&self.file_name).display()
        )?;
        write!(f, "\n")?;
        for base_dir in &self.base_dirs {
            write!(f, "* {}\n", base_dir.display())?;
        }
        write!(f, "\n")
    }
}
