#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Menu to allow the user to select game mode.

extern crate amethyst;
extern crate application_menu;
extern crate application_ui;
#[cfg(test)]
extern crate debug_util_amethyst;
#[macro_use]
extern crate derivative;
extern crate game_play;
#[macro_use]
extern crate log;
extern crate rayon;

pub use game_mode_menu_bundle::GameModeMenuBundle;
pub use index::Index;
pub use state::State;

mod game_mode_menu_bundle;
mod index;
mod menu_build_fn;
mod state;
mod system;
