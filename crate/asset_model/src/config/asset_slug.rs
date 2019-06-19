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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{AssetSlug, AssetSlugBuilder};
    use crate::config::{AssetSlugBuildError, AssetSlugSegment};

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
