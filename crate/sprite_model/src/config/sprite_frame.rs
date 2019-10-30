use std::convert::AsRef;

use derive_new::new;
use sequence_model::config::Wait;
use serde::{Deserialize, Serialize};

use crate::config::SpriteRef;

/// Frame with a `SpriteRef`.
///
/// This is useful when the sequence does not need any other behaviour besides displaying a sprite.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default)]
pub struct SpriteFrame {
    /// Number of ticks to wait before the sequence switches to the next frame.
    pub wait: Wait,
    /// Sprite to render.
    pub sprite: SpriteRef,
}

impl AsRef<Wait> for SpriteFrame {
    fn as_ref(&self) -> &Wait {
        &self.wait
    }
}
