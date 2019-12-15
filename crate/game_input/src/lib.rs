#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides types to link control input into Amethyst.

pub use crate::{
    component::{ButtonInputControlled, ControllerInput, InputControlled, SharedInputControlled},
    game_input_bundle::GameInputBundle,
    system::{ControllerInputUpdateSystem, SharedControllerInputUpdateSystem},
};

mod component;
mod game_input_bundle;
mod system;
