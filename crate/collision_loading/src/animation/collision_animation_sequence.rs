use CollisionAnimationFrame;

/// Information for an animation sequence.
pub trait CollisionAnimationSequence {
    /// Type of the animation's frames.
    type Frame: CollisionAnimationFrame;
    /// Returns the frames that make up this animation sequence.
    fn frames(&self) -> &[Self::Frame];
}
