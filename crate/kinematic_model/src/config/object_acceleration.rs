use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{ObjectAccelerationKind, ObjectAccelerationValue};

/// Acceleration added to an object for movement.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields, default)]
pub struct ObjectAcceleration {
    /// Whether acceleration is applied once or continuously.
    pub kind: ObjectAccelerationKind,
    /// X acceleration value.
    pub x: ObjectAccelerationValue,
    /// Y acceleration value.
    pub y: ObjectAccelerationValue,
    /// Z acceleration value.
    pub z: ObjectAccelerationValue,
}
