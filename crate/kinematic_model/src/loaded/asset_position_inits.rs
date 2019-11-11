use asset_model::loaded::AssetId;
use slotmap::SparseSecondaryMap;

use crate::loaded::PositionInits;

/// `PositionInits`s for an asset.
pub type AssetPositionInits = SparseSecondaryMap<AssetId, PositionInits>;
