//! Types used to reference the loaded form of assets.

pub use self::{
    asset_id::AssetId, asset_id_mappings::AssetIdMappings, asset_item_ids::AssetItemIds,
    asset_type_mappings::AssetTypeMappings, item_id::ItemId, item_ids::ItemIds,
    slug_and_handle::SlugAndHandle,
};

mod asset_id;
mod asset_id_mappings;
mod asset_item_ids;
mod asset_type_mappings;
mod item_id;
mod item_ids;
mod slug_and_handle;
