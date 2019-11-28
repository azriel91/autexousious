use slotmap::SecondaryMap;

use crate::loaded::{AssetId, ItemIds};

/// `ItemIds`s for an asset.
pub type AssetItemIds = SecondaryMap<AssetId, ItemIds>;
