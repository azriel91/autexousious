//! Types representing a map, in a form more usable in game.

pub use self::{
    asset_map_bounds::AssetMapBounds,
    asset_margins::AssetMargins,
    map::{Map, MapHandle},
    margins::Margins,
};

mod asset_map_bounds;
mod asset_margins;
mod map;
mod margins;
