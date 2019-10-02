#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use application::FindContext;

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
