use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::InteractionsSequenceHandles;

/// Sequence of `InteractionsSequenceHandle`s for an asset.
pub type AssetInteractionsSequenceHandles = SecondaryMap<AssetId, InteractionsSequenceHandles>;
