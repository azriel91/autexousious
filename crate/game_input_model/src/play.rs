//! Contains data types used at runtime.

pub use self::{
    axis_move_event_data::AxisMoveEventData, button_input_controlled::ButtonInputControlled,
    control_action_event_data::ControlActionEventData, control_input_event::ControlInputEvent,
    controller_input::ControllerInput, game_input_model_error::GameInputModelError,
    input_controlled::InputControlled, move_direction::MoveDirection,
    normal_input_controlled::NormalInputControlled, shared_input_controlled::SharedInputControlled,
};

mod axis_move_event_data;
mod button_input_controlled;
mod control_action_event_data;
mod control_input_event;
mod controller_input;
mod game_input_model_error;
mod input_controlled;
mod move_direction;
mod normal_input_controlled;
mod shared_input_controlled;
