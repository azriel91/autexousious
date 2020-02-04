use crate::play::{AxisMoveEventData, ControlActionEventData};

/// Event indicating a change in `ControlInput`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ControlInputEvent {
    /// `Axis` value has changed.
    AxisMoved(AxisMoveEventData),
    /// `ControlAction` has been pressed.
    ControlActionPress(ControlActionEventData),
    /// `ControlAction` has been released.
    ControlActionRelease(ControlActionEventData),
}
