use std::fmt;

use crate::config::AssetSlugSegment;

/// Error when building an `AssetSlug`.
#[derive(Clone, Debug, PartialEq)]
pub enum AssetSlugBuildError {
    /// Value provided is empty.
    SegmentEmpty {
        /// Asset slug field segment.
        segment: AssetSlugSegment,
    },
    /// Value provided contains control char.
    SegmentContainsControlChar {
        /// Asset slug field segment.
        segment: AssetSlugSegment,
        /// Invalid value provided to build the segment.
        value: String,
    },
    /// Value provided contains whitespace.
    SegmentContainsWhitespace {
        /// Asset slug field segment.
        segment: AssetSlugSegment,
        /// Invalid value provided to build the segment.
        value: String,
    },
    /// Value provided contains forward slash.
    SegmentContainsForwardSlash {
        /// Asset slug field segment.
        segment: AssetSlugSegment,
        /// Invalid value provided to build the segment.
        value: String,
    },
    /// Value provided starts with numeric char.
    SegmentStartsWithNumericChar {
        /// Asset slug field segment.
        segment: AssetSlugSegment,
        /// Invalid value provided to build the segment.
        value: String,
    },
    /// No value was provided to build this segment.
    NoValueProvided {
        /// Asset slug field segment.
        segment: AssetSlugSegment,
    },
    /// Too many segments were provided when building.
    ///
    /// This variant only occurs when building from `FromStr`; when using the `AssetSlugBuilder`,
    /// providing a value twice will overwrite the previous value.
    InvalidSegmentCount {
        /// Invalid value provided to build the asset slug.
        value: String,
    },
}

impl AssetSlugBuildError {
    /// Returns the segment this error applies to.
    pub fn segment(&self) -> Option<AssetSlugSegment> {
        match self {
            AssetSlugBuildError::SegmentEmpty { segment }
            | AssetSlugBuildError::SegmentContainsControlChar { segment, .. }
            | AssetSlugBuildError::SegmentContainsWhitespace { segment, .. }
            | AssetSlugBuildError::SegmentContainsForwardSlash { segment, .. }
            | AssetSlugBuildError::SegmentStartsWithNumericChar { segment, .. }
            | AssetSlugBuildError::NoValueProvided { segment, .. } => Some(*segment),
            AssetSlugBuildError::InvalidSegmentCount { .. } => None,
        }
    }

    /// Returns the value provided to build this segment, if any.
    pub fn value(&self) -> Option<&str> {
        match self {
            AssetSlugBuildError::SegmentContainsControlChar { value, .. }
            | AssetSlugBuildError::SegmentContainsWhitespace { value, .. }
            | AssetSlugBuildError::SegmentContainsForwardSlash { value, .. }
            | AssetSlugBuildError::SegmentStartsWithNumericChar { value, .. }
            | AssetSlugBuildError::InvalidSegmentCount { value } => Some(value.as_str()),
            AssetSlugBuildError::SegmentEmpty { .. }
            | AssetSlugBuildError::NoValueProvided { .. } => None,
        }
    }
}

impl fmt::Display for AssetSlugBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetSlugBuildError::SegmentEmpty { segment } => {
                write!(f, "{} must not be empty.", segment)
            }
            AssetSlugBuildError::SegmentContainsControlChar { segment, value } => write!(
                f,
                "{} must not contain control character: `{}`.",
                segment,
                value
                    .chars()
                    .map(|c| c.escape_default().to_string())
                    .collect::<String>()
            ),
            AssetSlugBuildError::SegmentContainsWhitespace { segment, value } => {
                write!(f, "{} must not contain whitespace: `{}`.", segment, value)
            }
            AssetSlugBuildError::SegmentContainsForwardSlash { segment, value } => write!(
                f,
                "{} must not contain the '/' character: `{}`.",
                segment, value
            ),
            AssetSlugBuildError::SegmentStartsWithNumericChar { segment, value } => write!(
                f,
                "{} must not start with numeric character: `{}`.",
                segment, value
            ),
            AssetSlugBuildError::NoValueProvided { segment } => {
                write!(f, "{} is a required value.", segment)
            }
            AssetSlugBuildError::InvalidSegmentCount { value } => write!(
                f,
                "Expected exactly one `/` in asset slug string: `{}`.",
                value
            ),
        }
    }
}

impl From<AssetSlugBuildError> for String {
    fn from(asset_slug_build_error: AssetSlugBuildError) -> String {
        format!("{}", asset_slug_build_error)
    }
}

#[cfg(test)]
mod tests {
    use super::AssetSlugBuildError;
    use crate::config::AssetSlugSegment;

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
