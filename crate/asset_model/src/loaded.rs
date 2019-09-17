//! Types used to reference the loaded form of assets.

pub use self::{
    asset_slug_id::AssetSlugId, asset_slug_id_mappings::AssetSlugIdMappings,
    asset_type_mappings::AssetTypeMappings, slug_and_handle::SlugAndHandle,
};

mod asset_slug_id;
mod asset_slug_id_mappings;
mod asset_type_mappings;
mod slug_and_handle;
