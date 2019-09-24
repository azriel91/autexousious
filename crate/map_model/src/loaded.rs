//! Types representing a map, in a form more usable in game.

pub use self::{
    asset_layer_positions::AssetLayerPositions,
    asset_map_bounds::AssetMapBounds,
    asset_map_definition_handle::AssetMapDefinitionHandle,
    asset_margins::AssetMargins,
    layer_positions::LayerPositions,
    map::{Map, MapHandle},
    margins::Margins,
};

mod asset_layer_positions;
mod asset_map_bounds;
mod asset_map_definition_handle;
mod asset_margins;
mod layer_positions;
mod map;
mod margins;
