use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_new::new;
use serde::{Deserialize, Serialize};

/// Animation frame that displays a sprite.
#[derive(
    Clone, Component, Copy, Debug, Default, Deserialize, Hash, PartialEq, Eq, Serialize, new,
)]
pub struct SpriteRef {
    /// Sprite sheet number.
    ///
    /// Note: this will almost always differ from the sheet number when loaded
    /// into Amethyst.
    ///
    /// Amethyst uses a global texture id map, so this number will be relative
    /// to the offset allocated to the object that this sprite sheet belongs
    /// to.
    pub sheet: usize,
    /// Sprite number on the sprite sheet.
    pub index: usize,
}
