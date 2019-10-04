#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! State where game mode selection takes place.

pub use crate::{
    game_mode_selection_state::{
        GameModeSelectionState, GameModeSelectionStateBuilder, GameModeSelectionStateDelegate,
    },
    game_mode_selection_trans::GameModeSelectionTrans,
};

mod game_mode_selection_state;
mod game_mode_selection_trans;
