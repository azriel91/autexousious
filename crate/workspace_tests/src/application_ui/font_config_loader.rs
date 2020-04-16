#[cfg(test)]
mod tests {
    use std::{iter::FromIterator, path::PathBuf};

    use application::{AppDir, FindContext};
    use ron;

    use application_ui::FontConfigLoader;

    #[test]
    fn fails_with_useful_error_when_font_config_does_not_exist() {
        if let Some(find_context) =
            FontConfigLoader::load_path(AppDir::RESOURCES, "non_existent.ron")
                .unwrap_err()
                .as_error()
                .downcast_ref::<Box<FindContext>>()
        {
            assert_eq!("non_existent.ron", find_context.file_name);
        } else {
            // kcov-ignore-start
            panic!("Expected resource `Find` error containing `non_existent.ron`");
            // kcov-ignore-end
        }
    }

    #[test]
    fn fails_with_useful_error_when_font_config_fails_to_parse() {
        if let Some(_ron_error) = FontConfigLoader::load_path(
            PathBuf::from_iter(["src", "application_ui"].iter())
                .to_str()
                .expect("Expected path to be valid UTF8."),
            "bad_config.ron",
        )
        .unwrap_err()
        .as_error()
        .downcast_ref::<Box<ron::de::Error>>()
        {
            // pass
        } else {
            panic!("Expected resource deserialization error"); // kcov-ignore
        }
    }
}
