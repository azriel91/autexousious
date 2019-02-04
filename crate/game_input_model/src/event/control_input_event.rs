use crate::{AxisEventData, ControlActionEventData};

/// Event indicating a change in `ControlInput`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ControlInputEvent {
    /// `Axis` value has changed.
    Axis(AxisEventData),
    /// `ControlAction` value has changed.
    ControlAction(ControlActionEventData),
}
