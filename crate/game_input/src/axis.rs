use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// Control axes for objects.
#[derive(Clone, Copy, Debug, Display, Deserialize, Hash, PartialEq, Eq, Serialize)]
pub enum Axis {
    /// X axis, positive is to the right, negative is to the left.
    X,
    /// Z axis, positive is downwards, negative is upwards.
    Z,
}

// Required by Amethyst.
impl Default for Axis {
    fn default() -> Self {
        Axis::X
    }
}
