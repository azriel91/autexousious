use crate::BodyAnimationFrame;

/// Information for an animation sequence.
pub trait BodyAnimationSequence {
    /// Type of the animation's frames.
    type Frame: BodyAnimationFrame;
    /// Returns the frames that make up this animation sequence.
    fn frames(&self) -> &[Self::Frame];
}
