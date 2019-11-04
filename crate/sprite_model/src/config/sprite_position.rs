use amethyst::{
    core::math::Vector3,
    ecs::{storage::DenseVecStorage, Component},
};
use derive_new::new;
use serde::{Deserialize, Serialize};

/// Position of a sprite on a background.
#[derive(Clone, Component, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize, new)]
#[serde(default)]
pub struct SpritePosition {
    /// X coordinate of the sprite on the background.
    pub x: i32,
    /// Y coordinate of the sprite on the background.
    pub y: i32,
    /// Z coordinate of the sprite on the background.
    pub z: i32,
}

impl Into<Vector3<f32>> for SpritePosition {
    fn into(self) -> Vector3<f32> {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}
