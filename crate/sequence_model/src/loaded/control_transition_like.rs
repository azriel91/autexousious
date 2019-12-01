use crate::loaded::InputReaction;

/// Marks types that has a `InputReaction` in its composition.
pub trait ControlTransitionLike {
    /// Returns the underlying `InputReaction`s.
    fn input_reaction(&self) -> &InputReaction;
}

impl ControlTransitionLike for InputReaction {
    fn input_reaction(&self) -> &InputReaction {
        self
    }
}
