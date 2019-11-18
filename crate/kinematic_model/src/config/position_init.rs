use std::ops::{Add, AddAssign};

use amethyst::{
    core::math::Vector3,
    ecs::{storage::DenseVecStorage, Component},
};
use asset_model::ItemComponent;
use derive_new::new;
use serde::{Deserialize, Serialize};

/// Position initializer for an entity.
#[derive(Clone, Copy, Debug, Default, Deserialize, ItemComponent, PartialEq, Eq, Serialize, new)]
#[serde(default)]
#[storage(DenseVecStorage)]
pub struct PositionInit {
    /// Initial X coordinate.
    pub x: i32,
    /// Initial Y coordinate.
    pub y: i32,
    /// Initial Z coordinate.
    pub z: i32,
}

impl Add for PositionInit {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for PositionInit {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Into<Vector3<f32>> for PositionInit {
    fn into(self) -> Vector3<f32> {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}
