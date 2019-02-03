use amethyst::ecs::Entity;

use crate::ControlAction;

/// `ControlAction` controller event data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlActionEventData {
    /// `Entity` this control event applies to.
    pub entity: Entity,
    /// `Axis` whose value changed.
    pub control_action: ControlAction,
    /// New value for the control action input.
    pub value: bool,
}
