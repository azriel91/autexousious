#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use asset_model::config::{AssetSlug, AssetSlugBuildError, AssetSlugBuilder, AssetSlugSegment};

    #[test]
    fn namespace_must_be_specified() {
        assert_eq!(
            Err(AssetSlugBuildError::NoValueProvided {
                segment: AssetSlugSegment::Namespace
            }
            .to_string()),
            AssetSlugBuilder::default().build()
        );
    }

    #[test]
    fn namespace_must_not_be_empty() {
        assert_eq!(
            Err(AssetSlugBuildError::SegmentEmpty {
                segment: AssetSlugSegment::Namespace
            }
            .to_string()),
            AssetSlugBuilder::default()
                .namespace("".to_string())
                .build()
        );
    }

    #[test]
    fn namespace_must_not_contain_control_character() {
        assert_eq!(
            Err(AssetSlugBuildError::SegmentContainsControlChar {
                segment: AssetSlugSegment::Namespace,
                value: String::from("ab")
            }
            .to_string()),
            AssetSlugBuilder::default()
                .namespace("ab".to_string())
                .build()
        );
    }

    #[test]
    fn namespace_must_not_contain_whitespace() {
        assert_eq!(
            Err(AssetSlugBuildError::SegmentContainsWhitespace {
                segment: AssetSlugSegment::Namespace,
                value: String::from("a b")
            }
            .to_string()),
            AssetSlugBuilder::default()
                .namespace("a b".to_string())
                .build()
        );
    }

    #[test]
    fn namespace_must_not_contain_forward_slash() {
        assert_eq!(
            Err(AssetSlugBuildError::SegmentContainsForwardSlash {
                segment: AssetSlugSegment::Namespace,
                value: String::from("a/b")
            }
            .to_string()),
            AssetSlugBuilder::default()
                .namespace("a/b".to_string())
                .build()
        );
    }

    #[test]
    fn namespace_must_not_start_with_numeric_character() {
        assert_eq!(
            Err(AssetSlugBuildError::SegmentStartsWithNumericChar {
                segment: AssetSlugSegment::Namespace,
                value: String::from("1ab")
            }
            .to_string()),
            AssetSlugBuilder::default()
                .namespace("1ab".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_be_specified() {
        assert_eq!(
            Err(AssetSlugBuildError::NoValueProvided {
                segment: AssetSlugSegment::Name
            }
            .to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_not_be_empty() {
        assert_eq!(
            Err(AssetSlugBuildError::SegmentEmpty {
                segment: AssetSlugSegment::Name
            }
            .to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .name("".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_not_contain_control_character() {
        assert_eq!(
            Err(AssetSlugBuildError::SegmentContainsControlChar {
                segment: AssetSlugSegment::Name,
                value: String::from("ab")
            }
            .to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .name("ab".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_not_contain_whitespace() {
        assert_eq!(
            Err(AssetSlugBuildError::SegmentContainsWhitespace {
                segment: AssetSlugSegment::Name,
                value: String::from("a b")
            }
            .to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .name("a b".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_not_contain_forward_slash() {
        assert_eq!(
            Err(AssetSlugBuildError::SegmentContainsForwardSlash {
                segment: AssetSlugSegment::Name,
                value: String::from("a/b")
            }
            .to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .name("a/b".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_not_start_with_numeric_character() {
        assert_eq!(
            Err(AssetSlugBuildError::SegmentStartsWithNumericChar {
                segment: AssetSlugSegment::Name,
                value: String::from("1ab")
            }
            .to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .name("1ab".to_string())
                .build()
        );
    }

    #[test]
    fn namespace_and_name_can_contain_unicode() {
        let asset_slug = AssetSlugBuilder::default()
            .namespace("忠犬".to_string())
            .name("ハチ公".to_string())
            .build();

        assert!(asset_slug.is_ok());

        let asset_slug = asset_slug.unwrap();
        assert_eq!("忠犬", asset_slug.namespace);
        assert_eq!("ハチ公", asset_slug.name);
        assert_eq!("忠犬/ハチ公", format!("{}", asset_slug));
    }

    #[test]
    fn from_str_returns_ok_when_valid() {
        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            Ok(AssetSlug {
                namespace: "test".to_string(),
                name: "name".to_string()
            }), // kcov-ignore
            AssetSlug::from_str("test/name")
        );
    }

    #[test]
    fn from_str_returns_err_when_too_few_segments() {
        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            Err(AssetSlugBuildError::InvalidSegmentCount {
                value: String::from("test")
            }),
            AssetSlug::from_str("test")
        );
    }

    #[test]
    fn from_str_returns_err_when_too_many_segments() {
        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            Err(AssetSlugBuildError::InvalidSegmentCount {
                value: String::from("test/abc/def")
            }),
            AssetSlug::from_str("test/abc/def")
        );
    }

    #[test]
    fn from_str_returns_err_from_builder_when_invalid() {
        assert_eq!(
            Err(AssetSlugBuildError::SegmentContainsWhitespace {
                segment: AssetSlugSegment::Name,
                value: String::from("a b")
            }),
            AssetSlug::from_str("test/a b")
        );
    }
}
