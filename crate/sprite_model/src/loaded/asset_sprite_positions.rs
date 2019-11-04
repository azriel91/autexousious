use asset_model::loaded::AssetId;
use slotmap::SparseSecondaryMap;

use crate::loaded::SpritePositions;

/// `SpritePositions`s for an asset.
pub type AssetSpritePositions = SparseSecondaryMap<AssetId, SpritePositions>;
