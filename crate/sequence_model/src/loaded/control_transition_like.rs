use crate::loaded::ControlTransition;

/// Marks types that has a `ControlTransition` in its composition.
pub trait ControlTransitionLike {
    /// Returns the underlying `ControlTransition`s.
    fn control_transition(&self) -> &ControlTransition;
}

impl ControlTransitionLike for ControlTransition {
    fn control_transition(&self) -> &ControlTransition {
        self
    }
}
