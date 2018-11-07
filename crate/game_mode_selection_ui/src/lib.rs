#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! UI to allow the user to select the game mode.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test;
extern crate application_menu;
extern crate application_ui;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
extern crate game_mode_selection_model;
extern crate heck;
#[macro_use]
extern crate log;
extern crate strum;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use game_mode_selection_ui_build_fn::GameModeSelectionUiBuildFn;
pub use game_mode_selection_ui_bundle::GameModeSelectionUiBundle;
pub(crate) use system::UiEventHandlerSystem;

mod game_mode_selection_ui_build_fn;
mod game_mode_selection_ui_bundle;
mod system;
