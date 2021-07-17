use serde::{Deserialize, Serialize};

/// Mode that this entity chases the target object.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum ChaseMode {
    /// Sticks to the target object.
    ///
    /// The `Position` and `Transform` of this entity is copied from the target
    /// entity.
    Stick,
}
