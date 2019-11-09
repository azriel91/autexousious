use asset_model::loaded::AssetId;
use slotmap::SparseSecondaryMap;

use crate::loaded::TintSequenceHandles;

/// `TintSequenceHandles` for an asset.
pub type AssetTintSequenceHandles = SparseSecondaryMap<AssetId, TintSequenceHandles>;
