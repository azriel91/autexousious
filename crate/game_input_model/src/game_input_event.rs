use amethyst::input::InputEvent;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::config::{ControlBindings, PlayerActionControl, PlayerAxisControl};

/// Input events that came through a device, whether local or remote.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum GameInputEvent {
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

impl TryFrom<InputEvent<ControlBindings>> for GameInputEvent {
    type Error = InputEvent<ControlBindings>;

    fn try_from(input_event: InputEvent<ControlBindings>) -> Result<Self, Self::Error> {
        match input_event {
            InputEvent::AxisMoved { axis, value } => Ok(GameInputEvent::AxisMoved { axis, value }),
            InputEvent::ActionPressed(player_axis_control) => {
                Ok(GameInputEvent::ActionPressed(player_axis_control))
            }
            InputEvent::ActionReleased(player_axis_control) => {
                Ok(GameInputEvent::ActionReleased(player_axis_control))
            }
            _ => Err(input_event),
        }
    }
}

impl From<GameInputEvent> for InputEvent<ControlBindings> {
    fn from(network_input_event: GameInputEvent) -> Self {
        match network_input_event {
            GameInputEvent::AxisMoved { axis, value } => InputEvent::AxisMoved { axis, value },
            GameInputEvent::ActionPressed(player_axis_control) => {
                InputEvent::ActionPressed(player_axis_control)
            }
            GameInputEvent::ActionReleased(player_axis_control) => {
                InputEvent::ActionReleased(player_axis_control)
            }
        }
    }
}
