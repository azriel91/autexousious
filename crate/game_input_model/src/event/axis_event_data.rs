use amethyst::ecs::Entity;

use crate::Axis;

/// `Axis` controller event data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxisEventData {
    /// `Entity` this control event applies to.
    pub entity: Entity,
    /// `Axis` whose value changed.
    pub axis: Axis,
    /// New value for the axis input.
    pub value: f64,
}
