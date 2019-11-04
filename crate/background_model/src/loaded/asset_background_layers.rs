use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::BackgroundLayers;

/// `BackgroundLayers` for an asset.
pub type AssetBackgroundLayers = SecondaryMap<AssetId, BackgroundLayers>;
