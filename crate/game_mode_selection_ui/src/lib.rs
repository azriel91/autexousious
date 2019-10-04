#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! UI to allow the user to select the game mode.

pub use crate::{
    game_mode_selection_ui_bundle::GameModeSelectionUiBundle,
    system::{
        GameModeSelectionSfxSystem, GameModeSelectionWidgetUiSystem, FONT_COLOUR_ACTIVE,
        FONT_COLOUR_IDLE,
    },
};

mod game_mode_selection_ui_bundle;
mod system;
