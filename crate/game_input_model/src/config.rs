//! Contains the types that represent the configuration on disk.

pub use self::{
    axis::Axis, control_action::ControlAction, control_args::ControlArgs,
    control_bindings::ControlBindings, control_input_event_args::ControlInputEventArgs,
    controller_config::ControllerConfig, controller_id::ControllerId, input_config::InputConfig,
    input_direction::InputDirection, input_direction_z::InputDirectionZ,
    player_action_control::PlayerActionControl, player_axis_control::PlayerAxisControl,
};

mod axis;
mod control_action;
mod control_args;
mod control_bindings;
mod control_input_event_args;
mod controller_config;
mod controller_id;
mod input_config;
mod input_direction;
mod input_direction_z;
mod player_action_control;
mod player_axis_control;
