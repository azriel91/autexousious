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
#[cfg(test)]
#[macro_use]
extern crate hamcrest;

#[macro_use]
extern crate serde_derive;

pub use crate::{
    axis::Axis,
    component::{ControllerInput, InputControlled, SharedInputControlled},
    config::{ControllerConfig, InputConfig},
    control_action::ControlAction,
    game_input_bundle::GameInputBundle,
    player_action_control::PlayerActionControl,
    player_axis_control::PlayerAxisControl,
    system::{ControllerInputUpdateSystem, SharedControllerInputUpdateSystem},
};

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
