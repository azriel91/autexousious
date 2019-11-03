use amethyst::ecs::{storage::DenseVecStorage, Component};
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
