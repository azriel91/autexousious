use serde::{Deserialize, Serialize};

/// Keys for special handling of asset preview widget layers.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum AswPortraitName {
    /// Portrait when the `AssetSelectionStatus` is `Inactive`.
    Join,
    /// Portrait when the `AssetSelectionStatus` is `InProgress` and AssetSelection is `Random`.
    Random,
    /// Portrait when the `AssetSelectionStatus` is `InProgress`.
    Select,
}
