#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides types to link control input into Amethyst.

pub use crate::{
    component::{ButtonInputControlled, ControllerInput, InputControlled, SharedInputControlled},
    system::{
        ControllerInputUpdateSystem, InputToControlInputSystem, SharedControllerInputUpdateSystem,
    },
};

mod component;
mod system;
