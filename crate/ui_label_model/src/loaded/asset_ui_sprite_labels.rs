use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::UiSpriteLabels;

/// `UiSpriteLabels`s for an asset.
pub type AssetUiSpriteLabels = SecondaryMap<AssetId, UiSpriteLabels>;
