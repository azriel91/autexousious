use amethyst::ecs::Entity;

use crate::config::{Axis, ControllerId};

/// `AxisMove` controller event data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxisMoveEventData {
    /// `ControllerId` that the input originated from.
    pub controller_id: ControllerId,
    /// `Entity` this control event applies to.
    pub entity: Entity,
    /// `Axis` whose value changed.
    pub axis: Axis,
    /// New value for the axis input.
    pub value: f32,
}
