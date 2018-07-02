#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Menu to allow the user to select game mode.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
extern crate application_menu;
extern crate application_ui;
extern crate character_selection;
#[cfg(test)]
extern crate debug_util_amethyst;
#[macro_use]
extern crate derivative;
extern crate game_play;
#[macro_use]
extern crate log;
extern crate rayon;

pub use game_mode_menu_bundle::GameModeMenuBundle;
pub use game_mode_menu_state::GameModeMenuState;
pub use index::Index;
pub(crate) use menu_build_fn::MenuBuildFn;
pub(crate) use ui_event_handler_system::UiEventHandlerSystem;

mod game_mode_menu_bundle;
mod game_mode_menu_state;
mod index;
mod menu_build_fn;
mod ui_event_handler_system;
