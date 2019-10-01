use std::{error, fmt, io};

use derivative::Derivative;
use derive_new::new;

use crate::io_support;

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
    #[derivative(PartialEq(compare_with = "io_support::cmp_opt_io_error"))]
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

#[cfg(test)]
mod test {
    use std::io;

    use super::DiscoveryContext;

    #[test]
    fn display_with_io_error() {
        let context = DiscoveryContext::new(
            "assets",
            "message",
            Some(io::Error::new(io::ErrorKind::Other, "boom")),
        );

        let expected = "\
                        Failed to find the 'assets' directory beside the current executable.\n\
                        Additional context:\n\
                        \n\
                        * **`io::Error`:** 'boom'\n\
                        * **Message:** 'message'\n\
                        \n";
        assert_eq!(expected, &format!("{}", context));
    }

    #[test]
    fn display_without_io_error() {
        let context = DiscoveryContext::new("assets", "message", None);

        let expected = "\
                        Failed to find the 'assets' directory beside the current executable.\n\
                        Additional context:\n\
                        \n\
                        * **Message:** 'message'\n\
                        \n";
        assert_eq!(expected, &format!("{}", context));
    }
}
