use amethyst::ecs::Entity;

use crate::ControlAction;

/// `ControlAction` controller event data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlActionEventData {
    /// `Entity` this control event applies to.
    pub entity: Entity,
    /// `ControlAction` whose value changed.
    pub control_action: ControlAction,
}
