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
extern crate derive_new;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use axis::Axis;
pub use control_action::ControlAction;
pub use player_action_control::PlayerActionControl;
pub use player_axis_control::PlayerAxisControl;

mod axis;
mod control_action;
mod player_action_control;
mod player_axis_control;

/// Type for Controller ID.
///
/// Better than stringly typed code.
pub type ControllerId = u32;
