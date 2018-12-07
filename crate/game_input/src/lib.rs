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




#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
#[cfg(test)]
#[macro_use]
extern crate hamcrest;

#[macro_use]
extern crate serde_derive;
use strum;
#[macro_use]
extern crate strum_macros;
use typename;
#[macro_use]
extern crate typename_derive;


pub use crate::axis::Axis;
pub use crate::component::{ControllerInput, InputControlled, SharedInputControlled};
pub use crate::config::{ControllerConfig, InputConfig};
pub use crate::control_action::ControlAction;
pub use crate::game_input_bundle::GameInputBundle;
pub use crate::player_action_control::PlayerActionControl;
pub use crate::player_axis_control::PlayerAxisControl;
pub use crate::system::{ControllerInputUpdateSystem, SharedControllerInputUpdateSystem};

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
