use crate::AnimationFrame;

/// Information for an animation sequence.
pub trait AnimationSequence {
    /// Type of the animation's frames.
    type Frame: AnimationFrame;
    /// Returns the frames that make up this animation sequence.
    fn frames(&self) -> &[Self::Frame];
}
