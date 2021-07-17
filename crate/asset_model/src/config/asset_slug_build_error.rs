use std::{cmp::PartialEq, fmt};

use derive_builder::UninitializedFieldError;

use crate::config::AssetSlugSegment;

/// Error when building an `AssetSlug`.
#[derive(Clone, Debug)]
pub enum AssetSlugBuildError {
    /// `derive_builder` UninitializedFieldError.
    UninitializedFieldError(UninitializedFieldError),
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
            Self::UninitializedFieldError(_) => None,
            Self::SegmentEmpty { segment }
            | Self::SegmentContainsControlChar { segment, .. }
            | Self::SegmentContainsWhitespace { segment, .. }
            | Self::SegmentContainsForwardSlash { segment, .. }
            | Self::SegmentStartsWithNumericChar { segment, .. }
            | Self::NoValueProvided { segment, .. } => Some(*segment),
            Self::InvalidSegmentCount { .. } => None,
        }
    }

    /// Returns the value provided to build this segment, if any.
    pub fn value(&self) -> Option<&str> {
        match self {
            Self::UninitializedFieldError(_) => None,
            Self::SegmentContainsControlChar { value, .. }
            | Self::SegmentContainsWhitespace { value, .. }
            | Self::SegmentContainsForwardSlash { value, .. }
            | Self::SegmentStartsWithNumericChar { value, .. }
            | Self::InvalidSegmentCount { value } => Some(value.as_str()),
            Self::SegmentEmpty { .. } | Self::NoValueProvided { .. } => None,
        }
    }
}

impl PartialEq for AssetSlugBuildError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::UninitializedFieldError(self_error),
                Self::UninitializedFieldError(other_error),
            ) => self_error.field_name() == other_error.field_name(),
            (
                Self::SegmentEmpty {
                    segment: self_segment,
                },
                Self::SegmentEmpty {
                    segment: other_segment,
                },
            ) => self_segment == other_segment,
            (
                Self::SegmentContainsControlChar {
                    segment: self_segment,
                    value: self_value,
                },
                Self::SegmentContainsControlChar {
                    segment: other_segment,
                    value: other_value,
                },
            )
            | (
                Self::SegmentContainsWhitespace {
                    segment: self_segment,
                    value: self_value,
                },
                Self::SegmentContainsWhitespace {
                    segment: other_segment,
                    value: other_value,
                },
            )
            | (
                Self::SegmentContainsForwardSlash {
                    segment: self_segment,
                    value: self_value,
                },
                Self::SegmentContainsForwardSlash {
                    segment: other_segment,
                    value: other_value,
                },
            )
            | (
                Self::SegmentStartsWithNumericChar {
                    segment: self_segment,
                    value: self_value,
                },
                Self::SegmentStartsWithNumericChar {
                    segment: other_segment,
                    value: other_value,
                },
            ) => self_segment == other_segment && self_value == other_value,
            (
                Self::NoValueProvided {
                    segment: self_segment,
                },
                Self::NoValueProvided {
                    segment: other_segment,
                },
            ) => self_segment == other_segment,
            (
                Self::InvalidSegmentCount { value: self_value },
                Self::InvalidSegmentCount { value: other_value },
            ) => self_value == other_value,
            _ => false,
        }
    }
}

impl fmt::Display for AssetSlugBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UninitializedFieldError(error) => {
                write!(f, "{} is uninitialized", error.field_name())
            }
            Self::SegmentEmpty { segment } => {
                write!(f, "{} must not be empty.", segment)
            }
            Self::SegmentContainsControlChar { segment, value } => write!(
                f,
                "{} must not contain control character: `{}`.",
                segment,
                value
                    .chars()
                    .map(|c| c.escape_default().to_string())
                    .collect::<String>()
            ),
            Self::SegmentContainsWhitespace { segment, value } => {
                write!(f, "{} must not contain whitespace: `{}`.", segment, value)
            }
            Self::SegmentContainsForwardSlash { segment, value } => write!(
                f,
                "{} must not contain the '/' character: `{}`.",
                segment, value
            ),
            Self::SegmentStartsWithNumericChar { segment, value } => write!(
                f,
                "{} must not start with numeric character: `{}`.",
                segment, value
            ),
            Self::NoValueProvided { segment } => {
                write!(f, "{} is a required value.", segment)
            }
            Self::InvalidSegmentCount { value } => write!(
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

impl From<UninitializedFieldError> for AssetSlugBuildError {
    fn from(error: UninitializedFieldError) -> Self {
        Self::UninitializedFieldError(error)
    }
}
