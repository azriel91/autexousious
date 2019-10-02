use std::{error, fmt, path::PathBuf};

/// Context of a find operation
///
/// Useful for debugging failures to find application configuration.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FindContext {
    /// Base directories searched during the find
    ///
    /// This includes:
    ///
    /// * Base directory of the executable
    pub base_dirs: Vec<PathBuf>,
    /// Directory relative to the base directories to look into
    pub conf_dir: PathBuf,
    /// Name of the file searched for
    pub file_name: String,
}

impl fmt::Display for FindContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Failed to find '{}' under any of the following directories:",
            self.conf_dir.join(&self.file_name).display()
        )?;
        writeln!(f)?;
        for base_dir in &self.base_dirs {
            writeln!(f, "* {}", base_dir.display())?;
        }
        writeln!(f)
    }
}

impl error::Error for FindContext {}
