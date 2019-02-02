#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used for control input.
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

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

pub use crate::{
    axis::Axis, control_action::ControlAction, controller_config::ControllerConfig,
    controller_id::ControllerId, input_config::InputConfig,
    player_action_control::PlayerActionControl, player_axis_control::PlayerAxisControl,
};

mod axis;
mod control_action;
mod controller_config;
mod controller_id;
mod input_config;
mod player_action_control;
mod player_axis_control;
