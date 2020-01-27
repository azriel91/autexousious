use amethyst::ecs::Entity;

use crate::{ControlAction, ControllerId};

/// `ControlAction` controller event data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlActionEventData {
    /// `ControllerId` that the input originated from.
    pub controller_id: ControllerId,
    /// `Entity` this control event applies to.
    pub entity: Entity,
    /// `ControlAction` whose value changed.
    pub control_action: ControlAction,
}
