//! Contains game play types for maps.

pub use self::{
    boundary_face::BoundaryFace, map_boundary_event::MapBoundaryEvent,
    map_boundary_event_data::MapBoundaryEventData, map_bounded::MapBounded,
    map_unbounded_delete::MapUnboundedDelete, out_of_bounds_delete_clock::OutOfBoundsDeleteClock,
};

mod boundary_face;
mod map_boundary_event;
mod map_boundary_event_data;
mod map_bounded;
mod map_unbounded_delete;
mod out_of_bounds_delete_clock;
