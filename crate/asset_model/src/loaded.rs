//! Types used to reference the loaded form of assets.

pub use self::{
    asset_slug_id::AssetSlugId, asset_slug_id_mappings::AssetSlugIdMappings,
    asset_slug_type_mappings::AssetSlugTypeMappings, slug_and_handle::SlugAndHandle,
};

mod asset_slug_id;
mod asset_slug_id_mappings;
mod asset_slug_type_mappings;
mod slug_and_handle;
