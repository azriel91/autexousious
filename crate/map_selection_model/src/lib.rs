#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during map selection.

pub use crate::{
    map_selection::MapSelection, map_selection_entity::MapSelectionEntity,
    map_selection_event::MapSelectionEvent,
};

mod map_selection;
mod map_selection_entity;
mod map_selection_event;
