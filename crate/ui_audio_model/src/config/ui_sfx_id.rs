use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

/// Logical IDs to reference audio used for UI.
#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, Hash, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum UiSfxId {
    /// Sound used for a `Cancel` action.
    Cancel,
    /// Sound used for a `Confirm` action.
    Confirm,
    /// Sound used for a `Deselect` action.
    Deselect,
    /// Sound used for a `Select` (soft confirm) action.
    Select,
    /// Sound used when switching between options.
    Switch,
}

// Necessary to allow enums to be in key position in TOML.
//
// See <https://github.com/serde-rs/serde/issues/908>.
//
// TODO: Do we still need this now that we are using YAML?
impl<'de> Deserialize<'de> for UiSfxId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}
