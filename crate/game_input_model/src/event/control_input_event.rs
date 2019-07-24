use crate::{AxisEventData, ControlActionEventData};

/// Event indicating a change in `ControlInput`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ControlInputEvent {
    /// `Axis` value has changed.
    Axis(AxisEventData),
    /// `ControlAction` has been pressed.
    ControlActionPressed(ControlActionEventData),
    /// `ControlAction` has been released.
    ControlActionReleased(ControlActionEventData),
}
