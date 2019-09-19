use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::ObjectAccelerationSequenceHandles;

/// Sequence of `ObjectAccelerationSequenceHandle`s for an asset.
pub type AssetObjectAccelerationSequenceHandles = SecondaryMap<AssetId, ObjectAccelerationSequenceHandles>;
