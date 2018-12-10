#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! UI to allow the user to select the game mode.

pub(crate) use crate::system::UiEventHandlerSystem;
pub use crate::{
    game_mode_selection_ui_build_fn::GameModeSelectionUiBuildFn,
    game_mode_selection_ui_bundle::GameModeSelectionUiBundle,
};

mod game_mode_selection_ui_build_fn;
mod game_mode_selection_ui_bundle;
mod system;
