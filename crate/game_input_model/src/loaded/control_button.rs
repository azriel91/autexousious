use crate::{config::ControlAction, loaded::ControlAxis};

/// Enum representing all possible control buttons.
///
/// This is not used in `InputConfig`, but as a logical representation
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ControlButton {
    /// `ControlAxis` button.
    Axis(ControlAxis),
    /// `ControlAction` button.
    Action(ControlAction),
}
