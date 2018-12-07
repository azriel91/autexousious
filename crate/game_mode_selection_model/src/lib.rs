#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used during game mode selection.



use strum;
#[macro_use]
extern crate strum_macros;

pub use crate::game_mode_index::GameModeIndex;
pub use crate::game_mode_selection_entity::GameModeSelectionEntity;
pub use crate::game_mode_selection_entity_id::GameModeSelectionEntityId;
pub use crate::game_mode_selection_event::GameModeSelectionEvent;

mod game_mode_index;
mod game_mode_selection_entity;
mod game_mode_selection_entity_id;
mod game_mode_selection_event;
