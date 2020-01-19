//! Types representing asset configuration.

pub use self::{
    asset_selection_event_args::AssetSelectionEventArgs,
    asset_selection_event_command::AssetSelectionEventCommand,
    asset_slug::{AssetSlug, AssetSlugBuilder},
    asset_slug_build_error::AssetSlugBuildError,
    asset_slug_segment::AssetSlugSegment,
    asset_slug_visitor::AssetSlugVisitor,
    asset_switch::AssetSwitch,
    asset_type::{AssetType, AssetTypeVariant},
    index::{AssetIndex, AssetRecord},
};

mod asset_selection_event_args;
mod asset_selection_event_command;
mod asset_slug;
mod asset_slug_build_error;
mod asset_slug_segment;
mod asset_slug_visitor;
mod asset_switch;
mod asset_type;
mod index;
