#[cfg(test)]
mod test {
    use std::io;

    use application::DiscoveryContext;

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
