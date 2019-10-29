use derive_new::new;
use sequence_model::config::Wait;
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteRef;

/// Components to use on this frame.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default)]
pub struct LayerFrame {
    /// Number of ticks to wait before the sequence switches to the next frame.
    pub wait: Wait,
    /// Sprite to render.
    pub sprite: SpriteRef,
}
