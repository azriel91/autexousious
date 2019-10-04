use std::{io, marker::PhantomData};

/// Functions to approximately compare two `io::Error`s.
#[derive(Debug)]
pub struct IoSupport(
    // Prevent instantiation.
    PhantomData<()>,
);

impl IoSupport {
    /// Returns an approximation of whether two `io::Error`s are equivalent.
    ///
    /// This is to make testing more ergonomic. The standard library does not implement `PartialEq`
    /// because it cannot be completely accurate. See:
    /// <https://github.com/rust-lang/rust/issues/34158>.
    pub fn cmp_io_error(expected: &io::Error, actual: &io::Error) -> bool {
        expected.kind() == actual.kind()
    }

    /// Returns an approximation of whether two `io::Error`s are equivalent.
    ///
    /// This is to make testing more ergonomic. The standard library does not implement `PartialEq`
    /// because it cannot be completely accurate. See:
    /// <https://github.com/rust-lang/rust/issues/34158>.
    pub fn cmp_io_error_opt(expected: &Option<io::Error>, actual: &Option<io::Error>) -> bool {
        if expected.is_none() && actual.is_none() {
            return true;
        }

        if (expected.is_some() && actual.is_none()) || (expected.is_none() && actual.is_some()) {
            return false;
        }

        Self::cmp_io_error(expected.as_ref().unwrap(), actual.as_ref().unwrap())
    }
}
