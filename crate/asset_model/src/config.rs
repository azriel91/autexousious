//! Types representing asset configuration.

pub use self::{
    asset_slug::{AssetSlug, AssetSlugBuilder},
    asset_slug_build_error::AssetSlugBuildError,
    asset_slug_segment::AssetSlugSegment,
    asset_slug_visitor::AssetSlugVisitor,
    asset_type::{AssetType, AssetTypeVariant},
    index::{AssetIndex, AssetRecord},
};

mod asset_slug;
mod asset_slug_build_error;
mod asset_slug_segment;
mod asset_slug_visitor;
pub mod asset_type;
mod index;
