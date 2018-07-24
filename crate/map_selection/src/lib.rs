#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Menu to allow the user to select game mode.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
extern crate game_model;
#[macro_use]
extern crate log;
extern crate map_model;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use map_selection::MapSelection;
pub use map_selection_event::MapSelectionEvent;
pub use map_selection_state::MapSelectionState;
pub(crate) use map_selection_system::MapSelectionSystem;
pub(crate) use selection_status::SelectionStatus;

mod map_selection;
mod map_selection_event;
mod map_selection_state;
mod map_selection_system;
mod selection_status;
