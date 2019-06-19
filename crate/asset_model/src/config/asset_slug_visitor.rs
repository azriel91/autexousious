use std::{fmt, str::FromStr};

use serde::de::{self, Visitor};

use crate::config::AssetSlug;

/// Visitor to deserialize an `AssetSlug` from a string.
#[derive(Debug)]
pub struct AssetSlugVisitor;

impl<'de> Visitor<'de> for AssetSlugVisitor {
    type Value = AssetSlug;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an asset slug such as `default/fireball`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        AssetSlug::from_str(value).map_err(de::Error::custom)
    }
}
