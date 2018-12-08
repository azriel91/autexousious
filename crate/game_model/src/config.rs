//! Types representing asset configuration.

pub use self::{
    asset_slug::{AssetSlug, AssetSlugBuilder},
    config_type::ConfigType,
    index::{AssetIndex, AssetRecord},
};

mod asset_slug;
mod config_type;
mod index;
