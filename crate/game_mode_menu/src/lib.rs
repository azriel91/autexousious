#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Menu to allow the user to select game mode.

extern crate amethyst;
extern crate application_input;
extern crate application_menu;
extern crate application_ui;
#[macro_use]
extern crate derivative;
#[cfg(test)]
extern crate enigo;
#[macro_use]
extern crate log;
extern crate rayon;
#[cfg(test)]
extern crate rayon_core;
#[cfg(test)]
extern crate winit;

pub use bundle::Bundle;
pub use index::Index;
pub use state::State;

mod bundle;
mod index;
mod menu_build_fn;
mod state;
mod system;
