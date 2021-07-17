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
    /// This variant only occurs when building from `FromStr`; when using the
    /// `AssetSlugBuilder`, providing a value twice will overwrite the
    /// previous value.
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
