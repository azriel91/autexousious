use asset_model::loaded::AssetId;
use slotmap::SparseSecondaryMap;

use crate::loaded::LayerPositions;

/// `LayerPositions`s for an asset.
pub type AssetLayerPositions = SparseSecondaryMap<AssetId, LayerPositions>;
