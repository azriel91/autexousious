use crate::{Axis, AxisMoveEventData};

/// Directional input variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveDirection {
    /// There is no movement.
    None,
    /// Axis input is up.
    Up,
    /// Axis input is down.
    Down,
    /// Axis input is left.
    Left,
    /// Axis input is right.
    Right,
}

impl From<AxisMoveEventData> for MoveDirection {
    fn from(axis_move_event_data: AxisMoveEventData) -> Self {
        let axis_value = axis_move_event_data.value;
        match axis_move_event_data.axis {
            Axis::X if axis_value > 0. => MoveDirection::Right,
            Axis::X if axis_value < 0. => MoveDirection::Left,
            Axis::Z if axis_value > 0. => MoveDirection::Up,
            Axis::Z if axis_value < 0. => MoveDirection::Down,
            _ => MoveDirection::None,
        }
    }
}
