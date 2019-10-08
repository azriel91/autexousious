use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::LoadStage;

/// `LoadStage` for each asset asset by ID.
pub type AssetLoadStage = SecondaryMap<AssetId, LoadStage>;
