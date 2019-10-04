#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides the bridging logic to feed UI input to the `game_input` logic.

pub use crate::{game_input_ui_bundle::GameInputUiBundle, system::InputToControlInputSystem};

mod game_input_ui_bundle;
mod system;
