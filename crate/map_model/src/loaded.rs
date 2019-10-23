//! Types representing a map, in a form more usable in game.

pub use self::{
    asset_map_bounds::AssetMapBounds, asset_map_definition_handle::AssetMapDefinitionHandle,
    asset_margins::AssetMargins, margins::Margins,
};

mod asset_map_bounds;
mod asset_map_definition_handle;
mod asset_margins;
mod margins;
