use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::LoadStatus;

/// `LoadStatus` for each asset by ID.
pub type AssetLoadStatus = SecondaryMap<AssetId, LoadStatus>;
