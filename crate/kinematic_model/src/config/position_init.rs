use amethyst::core::math::Vector3;
use derive_new::new;
use serde::{Deserialize, Serialize};

/// Position initializer for an entity.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize, new)]
#[serde(default)]
pub struct PositionInit {
    /// Initial X coordinate.
    pub x: i32,
    /// Initial Y coordinate.
    pub y: i32,
    /// Initial Z coordinate.
    pub z: i32,
}

impl Into<Vector3<f32>> for PositionInit {
    fn into(self) -> Vector3<f32> {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}
