use derive_new::new;
use serde::{Deserialize, Serialize};
use sprite_loading::AnimationFrame;
use sprite_model::config::SpriteRef;

/// Components to use on this frame.
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(default)]
pub struct LayerFrame {
    /// Number of ticks to wait before the sequence switches to the next frame.
    pub wait: u32,
    /// Sprite to render.
    pub sprite: SpriteRef,
}

impl AnimationFrame for LayerFrame {
    fn texture_index(&self) -> usize {
        self.sprite.sheet
    }

    fn sprite_index(&self) -> usize {
        self.sprite.index
    }

    fn wait(&self) -> u32 {
        self.wait
    }
}
