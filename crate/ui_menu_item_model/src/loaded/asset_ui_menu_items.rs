use asset_model::loaded::AssetId;
use slotmap::SecondaryMap;

use crate::loaded::UiMenuItems;

/// `UiMenuItems`s for an asset.
pub type AssetUiMenuItems<I> = SecondaryMap<AssetId, UiMenuItems<I>>;
