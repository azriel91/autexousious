use amethyst::input::InputEvent;
use game_input_model::config::{ControlBindings, PlayerActionControl, PlayerAxisControl};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Input events that came through the network.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum NetworkInputEvent {
    /// An axis value changed.
    AxisMoved {
        /// Axis whose value changed.
        axis: PlayerAxisControl,
        /// New value of the axis.
        value: f32,
    },
    /// An action button (or combination) was just pressed.
    ActionPressed(PlayerActionControl),
    /// An action button (or combination) was just released.
    ActionReleased(PlayerActionControl),
}

impl TryFrom<InputEvent<ControlBindings>> for NetworkInputEvent {
    type Error = InputEvent<ControlBindings>;

    fn try_from(input_event: InputEvent<ControlBindings>) -> Result<Self, Self::Error> {
        match input_event {
            InputEvent::AxisMoved { axis, value } => {
                Ok(NetworkInputEvent::AxisMoved { axis, value })
            }
            InputEvent::ActionPressed(player_axis_control) => {
                Ok(NetworkInputEvent::ActionPressed(player_axis_control))
            }
            InputEvent::ActionReleased(player_axis_control) => {
                Ok(NetworkInputEvent::ActionReleased(player_axis_control))
            }
            _ => Err(input_event),
        }
    }
}

impl From<NetworkInputEvent> for InputEvent<ControlBindings> {
    fn from(network_input_event: NetworkInputEvent) -> Self {
        match network_input_event {
            NetworkInputEvent::AxisMoved { axis, value } => InputEvent::AxisMoved { axis, value },
            NetworkInputEvent::ActionPressed(player_axis_control) => {
                InputEvent::ActionPressed(player_axis_control)
            }
            NetworkInputEvent::ActionReleased(player_axis_control) => {
                InputEvent::ActionReleased(player_axis_control)
            }
        }
    }
}
