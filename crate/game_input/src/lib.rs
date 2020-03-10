#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides types to link control input into Amethyst.

pub use crate::system::{
    ControllerInputUpdateSystem, GameInputToControlInputSystem, GameInputToControlInputSystemDesc,
    SharedControllerInputUpdateSystem,
};

mod system;
