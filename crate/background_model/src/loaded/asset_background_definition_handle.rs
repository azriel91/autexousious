use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::config::BackgroundDefinitionHandle;

/// `BackgroundDefinitionHandle` for an asset.
pub type AssetBackgroundDefinitionHandle = SecondaryMap<AssetId, BackgroundDefinitionHandle>;
