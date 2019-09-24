use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::WaitSequenceHandles;

/// Sequence of `WaitSequenceHandle`s for an asset.
pub type AssetWaitSequenceHandles = SecondaryMap<AssetId, WaitSequenceHandles>;
