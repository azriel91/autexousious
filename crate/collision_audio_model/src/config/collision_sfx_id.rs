use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

/// Logical IDs to reference audio used for collision.
#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, Hash, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CollisionSfxId {
    /// `Interaction` hit a normal `Body`.
    HitNormal,
}

// Necessary to allow enums to be in key position in YAML.
//
// See <https://github.com/serde-rs/serde/issues/908>.
impl<'de> Deserialize<'de> for CollisionSfxId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}
