#[cfg(test)]
mod tests {
    use asset_model::config::{AssetSlugBuildError, AssetSlugSegment};

    #[test]
    fn namespace_must_be_specified() {
        assert_eq!(
            "Namespace is a required value.",
            String::from(AssetSlugBuildError::NoValueProvided {
                segment: AssetSlugSegment::Namespace
            })
        );
    }

    #[test]
    fn namespace_must_not_be_empty() {
        assert_eq!(
            "Namespace must not be empty.",
            String::from(AssetSlugBuildError::SegmentEmpty {
                segment: AssetSlugSegment::Namespace
            })
        );
    }

    #[test]
    fn namespace_must_not_contain_control_character() {
        assert_eq!(
            "Namespace must not contain control character: `a\\u{9c}b`.",
            String::from(AssetSlugBuildError::SegmentContainsControlChar {
                segment: AssetSlugSegment::Namespace,
                value: String::from("ab")
            })
        );
    }

    #[test]
    fn namespace_must_not_start_with_numeric_character() {
        assert_eq!(
            "Namespace must not start with numeric character: `1ab`.",
            String::from(AssetSlugBuildError::SegmentStartsWithNumericChar {
                segment: AssetSlugSegment::Namespace,
                value: String::from("1ab")
            })
        );
    }

    #[test]
    fn namespace_must_not_contain_whitespace() {
        assert_eq!(
            "Namespace must not contain whitespace: `a b`.",
            String::from(AssetSlugBuildError::SegmentContainsWhitespace {
                segment: AssetSlugSegment::Namespace,
                value: String::from("a b")
            })
        );
    }

    #[test]
    fn namespace_must_not_contain_forward_slash() {
        assert_eq!(
            "Namespace must not contain the '/' character: `a/b`.",
            String::from(AssetSlugBuildError::SegmentContainsForwardSlash {
                segment: AssetSlugSegment::Namespace,
                value: String::from("a/b")
            })
        );
    }

    #[test]
    fn name_must_be_specified() {
        assert_eq!(
            "Name is a required value.",
            String::from(AssetSlugBuildError::NoValueProvided {
                segment: AssetSlugSegment::Name
            })
        );
    }

    #[test]
    fn name_must_not_be_empty() {
        assert_eq!(
            "Name must not be empty.",
            String::from(AssetSlugBuildError::SegmentEmpty {
                segment: AssetSlugSegment::Name
            })
        );
    }

    #[test]
    fn name_must_not_contain_control_character() {
        assert_eq!(
            "Name must not contain control character: `a\\u{9c}b`.",
            String::from(AssetSlugBuildError::SegmentContainsControlChar {
                segment: AssetSlugSegment::Name,
                value: String::from("ab")
            })
        );
    }

    #[test]
    fn name_must_not_start_with_numeric_character() {
        assert_eq!(
            "Name must not start with numeric character: `1ab`.",
            String::from(AssetSlugBuildError::SegmentStartsWithNumericChar {
                segment: AssetSlugSegment::Name,
                value: String::from("1ab")
            })
        );
    }

    #[test]
    fn name_must_not_contain_whitespace() {
        assert_eq!(
            "Name must not contain whitespace: `a b`.",
            String::from(AssetSlugBuildError::SegmentContainsWhitespace {
                segment: AssetSlugSegment::Name,
                value: String::from("a b")
            })
        );
    }

    #[test]
    fn name_must_not_contain_forward_slash() {
        assert_eq!(
            "Name must not contain the '/' character: `a/b`.",
            String::from(AssetSlugBuildError::SegmentContainsForwardSlash {
                segment: AssetSlugSegment::Name,
                value: String::from("a/b")
            })
        );
    }

    #[test]
    fn from_str_returns_err_when_too_few_segments() {
        assert_eq!(
            "Expected exactly one `/` in asset slug string: `test`.",
            String::from(AssetSlugBuildError::InvalidSegmentCount {
                value: String::from("test")
            })
        );
    }

    #[test]
    fn from_str_returns_err_when_too_many_segments() {
        assert_eq!(
            "Expected exactly one `/` in asset slug string: `test/abc/def`.",
            String::from(AssetSlugBuildError::InvalidSegmentCount {
                value: String::from("test/abc/def")
            })
        );
    }

    #[test]
    fn from_str_returns_err_from_builder_when_invalid() {
        assert_eq!(
            "Name must not contain whitespace: `a b`.",
            String::from(AssetSlugBuildError::SegmentContainsWhitespace {
                segment: AssetSlugSegment::Name,
                value: String::from("a b")
            })
        );
    }
}
