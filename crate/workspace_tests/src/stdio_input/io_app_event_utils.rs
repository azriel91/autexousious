#[cfg(test)]
mod tests {
    use application_event::AppEventVariant;

    use stdio_input::IoAppEventUtils;

    #[test]
    fn maps_shell_words_error_to_readable_string() {
        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            Err("Error splitting input string. Input:\n\
                 \n\
                 ```\n\
                 single quote \"\n\
                 ```\n\
                 \n\
                 Error:\n\
                 ```\n\
                 missing closing quote\n\
                 ```\n\n"
                .to_string()),
            IoAppEventUtils::input_to_variant_and_tokens("single quote \"")
        );
    }

    #[test]
    fn returns_ok_none_when_input_is_empty() {
        assert_eq!(Ok(None), IoAppEventUtils::input_to_variant_and_tokens(""));
    }

    #[test]
    fn returns_app_event_variants_and_all_tokens_when_input_matches_variant() {
        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            Ok(Some((
                AppEventVariant::AssetSelection,
                vec!["asset_selection".to_string(), "confirm".to_string()]
            ))),
            IoAppEventUtils::input_to_variant_and_tokens("asset_selection confirm")
        );
    }

    #[test]
    fn returns_useful_error_message_when_input_matches_variant() {
        let result = IoAppEventUtils::input_to_variant_and_tokens("abc");
        assert!(result.is_err());
        assert!(result.unwrap_err().starts_with(
            "Error parsing `abc` as an AppEventVariant. Error: `Matching variant not found`.\n\
             Valid values are: "
        ));
    }
}
