use serde::{Deserialize, Serialize};

/// Keys for special handling of asset preview widget layers.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum CswLayerName {
    /// Main widget entity.
    Main,
    /// Entity to display portrait.
    ///
    /// Also displays `press_to_join` message.
    Portrait,
}
