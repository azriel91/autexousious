#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Menu to allow the user to select game mode.

extern crate amethyst;
#[cfg(test)]
#[macro_use]
extern crate application;
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

pub use bundle::Bundle;
pub use index::Index;
pub use state::State;

mod bundle;
mod index;
mod menu_build_fn;
mod state;
mod system;
