//! Data types representing loaded configuration.

pub use self::{
    control_axis::ControlAxis, control_button::ControlButton, player_controller::PlayerController,
    player_controllers::PlayerControllers,
};

mod control_axis;
mod control_button;
mod player_controller;
mod player_controllers;
