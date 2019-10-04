use std::{char, fmt, str::FromStr};

use derive_builder::Builder;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::config::{AssetSlugBuildError, AssetSlugSegment, AssetSlugVisitor};

/// Namespaced reference to identify assets.
///
/// This should be constructed using `AssetSlugBuilder` as it performs validation on the values.
///
/// By convention, both the namespace and name should be lowercase and underscore separated. The
/// text displayed on character screens is a separate concept called the display name, read from
/// configuration.
///
/// **Developer's note:**
///
/// This is called a `Ref` instead of `Id` because the underlying asset may evolve, and the word
/// *ID* will likely be used to refer to a specific revision of the asset (deterministic).
///
/// # Examples
///
/// ```rust
/// use asset_model::config::{AssetSlug, AssetSlugBuilder};
///
/// fn main() -> Result<(), String> {
///     let asset_slug: AssetSlug = AssetSlugBuilder::default()
///         .namespace("azriel91".to_string())
///         .name("iris".to_string())
///         .build()?;
///
///     assert_eq!("azriel91/iris", format!("{}", asset_slug));
///
///     Ok(())
/// }
/// ```
#[derive(Builder, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[builder(derive(Debug), build_fn(validate = "Self::validate"))]
pub struct AssetSlug {
    // kcov-ignore-start
    /// Namespace of the asset, usually the username.
    pub namespace: String,
    /// Name of the asset, e.g. "iris".
    pub name: String,
    // kcov-ignore-end
}

impl AssetSlug {
    /// Serializes this `AssetSlug` as a single string, such as `default/fireball`.
    pub fn serialize_str<S>(asset_slug: &AssetSlug, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&asset_slug.to_string())
    }

    /// Deserializes this `AssetSlug` from a single string, such as `default/fireball`.
    pub fn deserialize_str<'de, D>(deserializer: D) -> Result<AssetSlug, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AssetSlugVisitor)
    }
}

impl AssetSlugBuilder {
    fn validate(&self) -> Result<(), AssetSlugBuildError> {
        Self::validate_segment(AssetSlugSegment::Namespace, &self.namespace)
            .and_then(|_| Self::validate_segment(AssetSlugSegment::Name, &self.name))
    }

    fn validate_segment(
        segment: AssetSlugSegment,
        value: &Option<String>,
    ) -> Result<(), AssetSlugBuildError> {
        if let Some(ref value) = value {
            if value.is_empty() {
                Err(AssetSlugBuildError::SegmentEmpty { segment })
            } else if value.contains(char::is_control) {
                // Put this validation first, because we don't want to print control characters in
                // any of the other validations.
                Err(AssetSlugBuildError::SegmentContainsControlChar {
                    segment,
                    value: value.to_string(),
                })
            } else if value.contains(char::is_whitespace) {
                Err(AssetSlugBuildError::SegmentContainsWhitespace {
                    segment,
                    value: value.to_string(),
                })
            } else if value.contains('/') {
                Err(AssetSlugBuildError::SegmentContainsForwardSlash {
                    segment,
                    value: value.to_string(),
                })
            } else if value.starts_with(char::is_numeric) {
                Err(AssetSlugBuildError::SegmentStartsWithNumericChar {
                    segment,
                    value: value.to_string(),
                })
            } else {
                Ok(())
            }
        } else {
            Err(AssetSlugBuildError::NoValueProvided { segment })
        }
    }
}

impl fmt::Display for AssetSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.namespace, self.name)
    }
}

impl FromStr for AssetSlug {
    type Err = AssetSlugBuildError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = s.split('/').collect();
        if segments.len() == 2 {
            let asset_slug_builder = AssetSlugBuilder {
                namespace: Some(segments[0].to_string()),
                name: Some(segments[1].to_string()),
            };
            asset_slug_builder.validate()?;

            // Deconstruct the builder.
            if let AssetSlugBuilder {
                namespace: Some(namespace),
                name: Some(name),
            } = asset_slug_builder
            {
                let asset_slug = AssetSlug { namespace, name };

                Ok(asset_slug)
            } else {
                unreachable!("`AssetSlugBuilder` fields have been previously set.");
            }
        } else {
            Err(AssetSlugBuildError::InvalidSegmentCount {
                value: s.to_string(),
            })
        }
    }
}
