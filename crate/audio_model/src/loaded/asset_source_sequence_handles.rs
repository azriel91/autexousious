use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::SourceSequenceHandles;

/// Sequence of `SourceSequenceHandle`s for an asset.
pub type AssetSourceSequenceHandles = SecondaryMap<AssetId, SourceSequenceHandles>;
