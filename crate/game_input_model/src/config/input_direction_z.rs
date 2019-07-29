use serde::{Deserialize, Serialize};

/// Variants for axis input matching.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum InputDirectionZ {
    /// Z axis input is zero.
    None,
    /// Z axis input is upwards.
    Up,
    /// Z axis input is downwards.
    Down,
    /// Axis input is non-zero.
    Some,
    /// Axis input is zero or opposite to the direction the character is facing.
    NotUp,
    /// Axis input is zero or in the same direction to the character is facing.
    NotDown,
}
