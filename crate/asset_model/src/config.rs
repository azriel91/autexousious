//! Types representing asset configuration.

pub use self::{
    asset_slug::{AssetSlug, AssetSlugBuilder},
    asset_slug_build_error::AssetSlugBuildError,
    asset_slug_segment::AssetSlugSegment,
    config_type::ConfigType,
    index::{AssetIndex, AssetRecord},
};

mod asset_slug;
mod asset_slug_build_error;
mod asset_slug_segment;
mod config_type;
mod index;
