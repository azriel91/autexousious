//! Types representing a map in configuration form.

pub use self::{
    layer::Layer,
    layer_frame::LayerFrame,
    layer_position::LayerPosition,
    map_bounds::MapBounds,
    map_definition::{MapDefinition, MapDefinitionHandle},
    map_header::MapHeader,
};

mod layer;
mod layer_frame;
mod layer_position;
mod map_bounds;
mod map_definition;
mod map_header;
