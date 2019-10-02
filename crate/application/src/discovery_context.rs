use std::{error, fmt, io};

use derivative::Derivative;
use derive_new::new;

use crate::IoSupport;

/// Information around the failure to discover a directory.
#[derive(Debug, Derivative, new)]
#[derivative(PartialEq)]
pub struct DiscoveryContext {
    // kcov-ignore-start
    /// Name of the directory under discovery.
    pub dir_name: &'static str,
    /// Human-readable message detailing what went wrong with the discovery
    pub message: &'static str,
    /// `io::Error` that caused this discovery error, if any.
    #[derivative(PartialEq(compare_with = "IoSupport::cmp_io_error_opt"))]
    pub io_error: Option<io::Error>,
    // kcov-ignore-end
}

impl fmt::Display for DiscoveryContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Failed to find the '{}' directory beside the current executable.",
            &self.dir_name
        )?;
        writeln!(f, "Additional context:\n")?;

        if let Some(ref io_error) = self.io_error {
            writeln!(f, "* **`io::Error`:** '{}'", &io_error)?;
        }
        writeln!(f, "* **Message:** '{}'\n", &self.message)
    }
}

impl error::Error for DiscoveryContext {}
