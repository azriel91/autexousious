#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Menu to allow the user to select game mode.

extern crate amethyst;
extern crate application_input;
extern crate application_ui;
#[cfg(test)]
extern crate enigo;
#[cfg(test)]
extern crate winit;

pub use state::State;

mod menu_build_fn;
mod state;
