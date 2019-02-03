#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides types to link control input into Amethyst.

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

pub use crate::{
    component::{ControllerInput, InputControlled, SharedInputControlled},
    game_input_bundle::GameInputBundle,
    system::{
        ControllerInputUpdateSystem, InputToControlInputSystem, SharedControllerInputUpdateSystem,
    },
};

mod component;
mod game_input_bundle;
mod system;
