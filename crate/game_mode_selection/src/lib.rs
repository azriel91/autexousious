#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! State where game mode selection takes place.

extern crate amethyst;
extern crate application_event;
extern crate application_menu;
extern crate application_state;
extern crate character_selection;
extern crate character_selection_ui;
#[cfg(test)]
extern crate debug_util_amethyst;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
extern crate game_loading;
extern crate game_mode_selection_model;
extern crate game_play;
#[macro_use]
extern crate log;
extern crate map_selection;
extern crate map_selection_ui;

pub use game_mode_selection_state::{
    GameModeSelectionState, GameModeSelectionStateBuilder, GameModeSelectionStateDelegate,
};
pub(crate) use game_mode_selection_trans::GameModeSelectionTrans;

mod game_mode_selection_state;
mod game_mode_selection_trans;
