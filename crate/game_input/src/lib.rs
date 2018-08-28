#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides types for game control input.
//!
//! Currently the planned input buttons are:
//!
//! * Up
//! * Down
//! * Left
//! * Right
//! * Defend
//! * Jump
//! * Attack
//! * Special

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
#[cfg(test)]
extern crate application;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate typename;
#[macro_use]
extern crate typename_derive;
#[cfg(test)]
extern crate winit;

pub use axis::Axis;
pub use component::{ControllerInput, InputControlled};
pub use config::{ControllerConfig, InputConfig};
pub use control_action::ControlAction;
pub use game_input_bundle::GameInputBundle;
pub use player_action_control::PlayerActionControl;
pub use player_axis_control::PlayerAxisControl;
pub use system::ControllerInputUpdateSystem;

mod axis;
mod component;
mod config;
mod control_action;
mod game_input_bundle;
mod player_action_control;
mod player_axis_control;
mod system;

/// Type for Controller ID.
///
/// Better than stringly typed code.
pub type ControllerId = u32;
