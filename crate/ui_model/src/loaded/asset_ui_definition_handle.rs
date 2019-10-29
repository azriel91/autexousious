use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::config::UiDefinitionHandle;

/// `UiDefinitionHandle` for an asset.
pub type AssetUiDefinitionHandle = SecondaryMap<AssetId, UiDefinitionHandle>;
