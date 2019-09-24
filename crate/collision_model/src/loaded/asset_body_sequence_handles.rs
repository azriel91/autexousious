use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::BodySequenceHandles;

/// Sequence of `BodySequenceHandle`s for an asset.
pub type AssetBodySequenceHandles = SecondaryMap<AssetId, BodySequenceHandles>;
