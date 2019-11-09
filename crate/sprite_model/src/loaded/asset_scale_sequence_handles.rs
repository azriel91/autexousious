use asset_model::loaded::AssetId;
use slotmap::SparseSecondaryMap;

use crate::loaded::ScaleSequenceHandles;

/// `ScaleSequenceHandles` for an asset.
pub type AssetScaleSequenceHandles = SparseSecondaryMap<AssetId, ScaleSequenceHandles>;
