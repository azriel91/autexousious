use std::ops::{Deref, DerefMut};

use amethyst::ecs::{storage::VecStorage, Component};
use serde::{Deserialize, Serialize};

/// Number of ticks to stay on the current frame before switching to the next frame.
#[derive(Clone, Component, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
#[storage(VecStorage)]
pub struct Scale(pub Option<f32>);

impl Scale {
    /// Returns a new `Scale` with the given value.
    pub fn new(value: f32) -> Self {
        Scale(Some(value))
    }
}

impl Deref for Scale {
    type Target = Option<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Scale {
    fn deref_mut(&mut self) -> &mut Option<f32> {
        &mut self.0
    }
}
