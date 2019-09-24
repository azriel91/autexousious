use asset_model::loaded::AssetId;
use slotmap::SparseSecondaryMap;

use crate::config::MapBounds;

/// `MapBounds`s for an asset.
pub type AssetMapBounds = SparseSecondaryMap<AssetId, MapBounds>;
