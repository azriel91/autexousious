use serde::{Deserialize, Serialize};

/// Keys for special handling of map selection widget layers.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum MswLayerName {
    /// Main widget entity.
    Main,
    /// Entity to display portrait.
    Portrait,
}
