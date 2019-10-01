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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::FindContext;

    #[test]
    #[cfg(not(windows))]
    fn display_outputs_nice_markdown() {
        let find_context = FindContext {
            base_dirs: vec![PathBuf::from("/base/zero"), PathBuf::from("/base/one")],
            conf_dir: PathBuf::from("resources"),
            file_name: "input.ron".to_string(),
        }; // kcov-ignore

        assert_eq!(
            "\
             Failed to find 'resources/input.ron' under any of the following directories:\n\
             \n\
             * /base/zero\n\
             * /base/one\n\
             \n",
            format!("{}", find_context)
        ); // kcov-ignore
    }

    #[test]
    #[cfg(windows)]
    fn display_outputs_nice_markdown() {
        let find_context = FindContext {
            base_dirs: vec![PathBuf::from("/base/zero"), PathBuf::from("/base/one")],
            conf_dir: PathBuf::from("resources"),
            file_name: "input.ron".to_string(),
        };

        assert_eq!(
            "\
             Failed to find 'resources\\input.ron' under any of the following directories:\n\
             \n\
             * /base/zero\n\
             * /base/one\n\
             \n",
            format!("{}", find_context)
        ); // kcov-ignore
    }
}
