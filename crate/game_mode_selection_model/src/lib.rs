#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during game mode selection.

pub use crate::{
    game_mode_index::GameModeIndex, game_mode_selection_entity::GameModeSelectionEntity,
    game_mode_selection_event::GameModeSelectionEvent,
};

mod game_mode_index;
mod game_mode_selection_entity;
mod game_mode_selection_event;
