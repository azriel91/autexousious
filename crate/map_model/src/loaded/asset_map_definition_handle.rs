use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::config::MapDefinitionHandle;

/// `MapDefinitionHandle` for an asset.
pub type AssetMapDefinitionHandle = SecondaryMap<AssetId, MapDefinitionHandle>;
