use sprite_loading::AnimationSequence;
use sprite_model::config::SpriteFrame;

use config::Position;

/// An image layer on a map.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, new)]
pub struct Layer {
    /// Position of the image on the map.
    #[serde(default)]
    pub position: Position,
    /// Key frames in the animation sequence.
    pub frames: Vec<SpriteFrame>,
}

impl AnimationSequence for Layer {
    type Frame = SpriteFrame;

    fn frames(&self) -> &[SpriteFrame] {
        &self.frames
    }
}
