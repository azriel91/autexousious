use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_new::new;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

use crate::config::{ObjectAccelerationKind, ObjectAccelerationValue};

/// Acceleration added to an object for movement.
#[derive(Clone, Component, Copy, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields, default)]
#[storage(DenseVecStorage)]
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
