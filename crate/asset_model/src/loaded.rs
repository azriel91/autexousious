//! Types used to reference the loaded form of assets.

pub use self::{
    asset_id::AssetId, asset_id_mappings::AssetIdMappings, asset_type_mappings::AssetTypeMappings,
    slug_and_handle::SlugAndHandle,
};

mod asset_id;
mod asset_id_mappings;
mod asset_type_mappings;
mod slug_and_handle;
