//! Types representing a map in configuration form.

pub use self::{
    map_bounds::MapBounds,
    map_definition::{MapDefinition, MapDefinitionHandle},
    map_header::MapHeader,
};

mod map_bounds;
mod map_definition;
mod map_header;
