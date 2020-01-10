#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

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

pub use crate::{
    axis::Axis,
    control_action::ControlAction,
    control_args::ControlArgs,
    control_bindings::ControlBindings,
    control_input_event_args::ControlInputEventArgs,
    controller_config::ControllerConfig,
    controller_id::ControllerId,
    event::{AxisMoveEventData, ControlActionEventData, ControlInputEvent},
    input_config::InputConfig,
    player_action_control::PlayerActionControl,
    player_axis_control::PlayerAxisControl,
};

pub mod config;

mod axis;
mod control_action;
mod control_args;
mod control_bindings;
mod control_input_event_args;
mod controller_config;
mod controller_id;
mod event;
mod input_config;
mod player_action_control;
mod player_axis_control;
