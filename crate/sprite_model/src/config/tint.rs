use derive_new::new;
use serde::{Deserialize, Serialize};

/// RGBA multipliers to apply to the sprite.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct Tint {
    /// Value between 0.0 and 1.0 (inclusive) to multiply the red channel with.
    pub r: f32,
    /// Value between 0.0 and 1.0 (inclusive) to multiply the green channel with.
    pub g: f32,
    /// Value between 0.0 and 1.0 (inclusive) to multiply the blue channel with.
    pub b: f32,
    /// Value between 0.0 and 1.0 (inclusive) to multiply the alpha channel with.
    pub a: f32,
}

impl Default for Tint {
    fn default() -> Self {
        Tint {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 1.,
        }
    }
}
