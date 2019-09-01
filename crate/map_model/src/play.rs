//! Contains game play types for maps.

pub use self::{
    boundary_face::BoundaryFace, map_boundary_event::MapBoundaryEvent,
    map_boundary_event_data::MapBoundaryEventData, map_bounded::MapBounded,
};

mod boundary_face;
mod map_boundary_event;
mod map_boundary_event_data;
mod map_bounded;
