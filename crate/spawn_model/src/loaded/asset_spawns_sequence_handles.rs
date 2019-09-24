use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::SpawnsSequenceHandles;

/// Sequence of `SpawnsSequenceHandle`s for an asset.
pub type AssetSpawnsSequenceHandles = SecondaryMap<AssetId, SpawnsSequenceHandles>;
