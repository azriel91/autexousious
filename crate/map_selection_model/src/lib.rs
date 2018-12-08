#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used during map selection.

pub use crate::{
    map_selection::MapSelection, map_selection_entity::MapSelectionEntity,
    map_selection_entity_id::MapSelectionEntityId, map_selection_event::MapSelectionEvent,
};

mod map_selection;
mod map_selection_entity;
mod map_selection_entity_id;
mod map_selection_event;
