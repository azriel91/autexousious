#[cfg(test)]
mod test {
    use std::io;

    use application::IoSupport;

    #[test]
    fn cmp_different_error_kind_equals_false() {
        assert!(!IoSupport::cmp_io_error(
            &io::Error::new(io::ErrorKind::PermissionDenied, "boom"),
            &io::Error::new(io::ErrorKind::Other, "boom")
        ));
    }

    #[test]
    fn cmp_same_error_kind_equals_true() {
        assert!(IoSupport::cmp_io_error(
            &io::Error::new(io::ErrorKind::PermissionDenied, "boom"),
            &io::Error::new(io::ErrorKind::PermissionDenied, "pow")
        ));
    }

    #[test]
    fn cmp_opt_both_none_equals_true() {
        assert!(IoSupport::cmp_io_error_opt(&None, &None));
    }

    #[test]
    fn cmp_opt_left_none_right_some_equals_false() {
        assert!(!IoSupport::cmp_io_error_opt(
            &None,
            &Some(io::Error::new(io::ErrorKind::Other, "boom"))
        ));
    }

    #[test]
    fn cmp_opt_left_some_right_none_equals_false() {
        assert!(!IoSupport::cmp_io_error_opt(
            &None,
            &Some(io::Error::new(io::ErrorKind::Other, "boom"))
        ));
    }

    #[test]
    fn cmp_opt_different_error_kind_equals_false() {
        assert!(!IoSupport::cmp_io_error_opt(
            &Some(io::Error::new(io::ErrorKind::PermissionDenied, "boom")),
            &Some(io::Error::new(io::ErrorKind::Other, "boom"))
        ));
    }

    #[test]
    fn cmp_opt_same_error_kind_equals_true() {
        assert!(IoSupport::cmp_io_error_opt(
            &Some(io::Error::new(io::ErrorKind::PermissionDenied, "boom")),
            &Some(io::Error::new(io::ErrorKind::PermissionDenied, "pow"))
        ));
    }
}
