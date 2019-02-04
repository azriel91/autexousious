#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the bridging logic to feed UI input to the `game_input` logic.

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

pub use crate::{game_input_ui_bundle::GameInputUiBundle, system::InputToControlInputSystem};

mod game_input_ui_bundle;
mod system;
