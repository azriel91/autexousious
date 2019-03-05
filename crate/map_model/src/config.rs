//! Types representing a map in configuration form.

pub use self::{
    layer::Layer, layer_frame::LayerFrame, map_bounds::MapBounds, map_definition::MapDefinition,
    map_header::MapHeader, position::Position,
};

mod layer;
mod layer_frame;
mod map_bounds;
mod map_definition;
mod map_header;
mod position;
