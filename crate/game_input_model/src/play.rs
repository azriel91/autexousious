//! Contains data types used at runtime.

pub use self::{
    button_input_controlled::ButtonInputControlled, controller_input::ControllerInput,
    input_controlled::InputControlled, move_direction::MoveDirection,
    shared_input_controlled::SharedInputControlled,
};

mod button_input_controlled;
mod controller_input;
mod input_controlled;
mod move_direction;
mod shared_input_controlled;
