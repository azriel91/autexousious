#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Menu to allow the user to select game mode.

extern crate amethyst;
extern crate application_input;
#[cfg(test)]
extern crate enigo;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate winit;

pub use state::State;

mod state;
