use std::char;
use std::fmt;
use std::str::FromStr;

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
/// use game_model::config::{AssetSlug, AssetSlugBuilder};
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
#[derive(Builder, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[builder(derive(Debug), build_fn(validate = "Self::validate"))]
pub struct AssetSlug {
    // kcov-ignore-start
    /// Namespace of the asset, usually the username.
    pub namespace: String,
    /// Name of the asset, e.g. "iris".
    pub name: String,
    // kcov-ignore-end
}

impl AssetSlugBuilder {
    fn validate(&self) -> Result<(), String> {
        Self::validate_segment("Asset namespace", &self.namespace)
            .and_then(|_| Self::validate_segment("Asset name", &self.name))
    }

    fn validate_segment(segment_name: &str, segment: &Option<String>) -> Result<(), String> {
        if let Some(ref value) = segment {
            if value.is_empty() {
                Err(format!("{} must not be empty.", segment_name))
            } else if value.contains(char::is_control) {
                // Put this validation first, because we don't want to print control characters in
                // any of the other validations.
                Err(format!(
                    "{} must not contain control character: `{}`.",
                    segment_name,
                    value
                        .chars()
                        .map(|c| c.escape_default().to_string())
                        .collect::<String>()
                ))
            } else if value.starts_with(char::is_numeric) {
                Err(format!(
                    "{} must not start with numeric character: `{}`.",
                    segment_name, value
                ))
            } else if value.contains(char::is_whitespace) {
                Err(format!(
                    "{} must not contain whitespace: `{}`.",
                    segment_name, value
                ))
            } else if value.contains('/') {
                Err(format!(
                    "{} must not contain the '/' character: `{}`.",
                    segment_name, value
                ))
            } else {
                Ok(())
            }
        } else {
            Err(format!("{} is a required value.", segment_name))
        }
    }
}

impl fmt::Display for AssetSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.namespace, self.name)
    }
}

impl FromStr for AssetSlug {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = s.split('/').collect();
        if segments.len() == 2 {
            AssetSlugBuilder::default()
                .namespace(segments[0].to_string())
                .name(segments[1].to_string())
                .build()
        } else {
            Err(format!("Expected exactly one `/` in slug string: {:?}.", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{AssetSlug, AssetSlugBuilder};

    #[test]
    fn namespace_must_be_specified() {
        assert_eq!(
            Err("Asset namespace is a required value.".to_string()),
            AssetSlugBuilder::default().build()
        );
    }

    #[test]
    fn namespace_must_not_be_empty() {
        assert_eq!(
            Err("Asset namespace must not be empty.".to_string()),
            AssetSlugBuilder::default()
                .namespace("".to_string())
                .build()
        );
    }

    #[test]
    fn namespace_must_not_contain_control_character() {
        assert_eq!(
            Err("Asset namespace must not contain control character: `a\\u{9c}b`.".to_string()),
            AssetSlugBuilder::default()
                .namespace("ab".to_string())
                .build()
        );
    }

    #[test]
    fn namespace_must_not_start_with_numeric_character() {
        assert_eq!(
            Err("Asset namespace must not start with numeric character: `1ab`.".to_string()),
            AssetSlugBuilder::default()
                .namespace("1ab".to_string())
                .build()
        );
    }

    #[test]
    fn namespace_must_not_contain_whitespace() {
        assert_eq!(
            Err("Asset namespace must not contain whitespace: `a b`.".to_string()),
            AssetSlugBuilder::default()
                .namespace("a b".to_string())
                .build()
        );
    }

    #[test]
    fn namespace_must_not_contain_forward_slash() {
        assert_eq!(
            Err("Asset namespace must not contain the '/' character: `a/b`.".to_string()),
            AssetSlugBuilder::default()
                .namespace("a/b".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_be_specified() {
        assert_eq!(
            Err("Asset name is a required value.".to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_not_be_empty() {
        assert_eq!(
            Err("Asset name must not be empty.".to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .name("".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_not_contain_control_character() {
        assert_eq!(
            Err("Asset name must not contain control character: `a\\u{9c}b`.".to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .name("ab".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_not_start_with_numeric_character() {
        assert_eq!(
            Err("Asset name must not start with numeric character: `1ab`.".to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .name("1ab".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_not_contain_whitespace() {
        assert_eq!(
            Err("Asset name must not contain whitespace: `a b`.".to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .name("a b".to_string())
                .build()
        );
    }

    #[test]
    fn name_must_not_contain_forward_slash() {
        assert_eq!(
            Err("Asset name must not contain the '/' character: `a/b`.".to_string()),
            AssetSlugBuilder::default()
                .namespace("test".to_string())
                .name("a/b".to_string())
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
            Err("Expected exactly one `/` in slug string: \"test\".".to_string()),
            AssetSlug::from_str("test")
        );
    }

    #[test]
    fn from_str_returns_err_when_too_many_segments() {
        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            Err("Expected exactly one `/` in slug string: \"test/abc/def\".".to_string()),
            AssetSlug::from_str("test/abc/def")
        );
    }

    #[test]
    fn from_str_returns_err_from_builder_when_invalid() {
        // kcov-ignore-start
        assert_eq!(
            // kcov-ignore-end
            Err("Asset name must not contain whitespace: `a b`.".to_string()),
            AssetSlug::from_str("test/a b")
        );
    }
}
