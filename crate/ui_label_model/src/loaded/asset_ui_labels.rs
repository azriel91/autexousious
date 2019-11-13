use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::config::UiLabels;

/// `UiLabels`s for an asset.
pub type AssetUiLabels = SecondaryMap<AssetId, UiLabels>;
