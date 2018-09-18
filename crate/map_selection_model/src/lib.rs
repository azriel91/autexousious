#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used during map selection.

extern crate game_model;
extern crate map_model;

pub use map_selection::MapSelection;
pub use map_selection_event::MapSelectionEvent;

mod map_selection;
mod map_selection_event;
