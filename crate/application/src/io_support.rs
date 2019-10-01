use std::io;

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
pub fn cmp_opt_io_error(expected: &Option<io::Error>, actual: &Option<io::Error>) -> bool {
    if expected.is_none() && actual.is_none() {
        return true;
    }

    if (expected.is_some() && actual.is_none()) || (expected.is_none() && actual.is_some()) {
        return false;
    }

    cmp_io_error(expected.as_ref().unwrap(), actual.as_ref().unwrap())
}

#[cfg(test)]
mod test {
    use std::io;

    use super::{cmp_io_error, cmp_opt_io_error};

    #[test]
    fn cmp_different_error_kind_equals_false() {
        assert!(!cmp_io_error(
            &io::Error::new(io::ErrorKind::PermissionDenied, "boom"),
            &io::Error::new(io::ErrorKind::Other, "boom")
        ));
    }

    #[test]
    fn cmp_same_error_kind_equals_true() {
        assert!(cmp_io_error(
            &io::Error::new(io::ErrorKind::PermissionDenied, "boom"),
            &io::Error::new(io::ErrorKind::PermissionDenied, "pow")
        ));
    }

    #[test]
    fn cmp_opt_both_none_equals_true() {
        assert!(cmp_opt_io_error(&None, &None));
    }

    #[test]
    fn cmp_opt_left_none_right_some_equals_false() {
        assert!(!cmp_opt_io_error(
            &None,
            &Some(io::Error::new(io::ErrorKind::Other, "boom"))
        ));
    }

    #[test]
    fn cmp_opt_left_some_right_none_equals_false() {
        assert!(!cmp_opt_io_error(
            &None,
            &Some(io::Error::new(io::ErrorKind::Other, "boom"))
        ));
    }

    #[test]
    fn cmp_opt_different_error_kind_equals_false() {
        assert!(!cmp_opt_io_error(
            &Some(io::Error::new(io::ErrorKind::PermissionDenied, "boom")),
            &Some(io::Error::new(io::ErrorKind::Other, "boom"))
        ));
    }

    #[test]
    fn cmp_opt_same_error_kind_equals_true() {
        assert!(cmp_opt_io_error(
            &Some(io::Error::new(io::ErrorKind::PermissionDenied, "boom")),
            &Some(io::Error::new(io::ErrorKind::PermissionDenied, "pow"))
        ));
    }
}
