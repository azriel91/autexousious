use crate::InteractionAnimationFrame;

/// Information for an animation sequence.
pub trait InteractionAnimationSequence {
    /// Type of the animation's frames.
    type Frame: InteractionAnimationFrame;
    /// Returns the frames that make up this animation sequence.
    fn frames(&self) -> &[Self::Frame];
}
