use crate::{config::SequenceId, loaded::ControlTransition};

/// Marks types that has a `ControlTransition` in its composition.
pub trait ControlTransitionLike<SeqId>
where
    SeqId: SequenceId,
{
    /// Returns the underlying `ControlTransition`s.
    fn control_transition(&self) -> &ControlTransition<SeqId>;
}

impl<SeqId> ControlTransitionLike<SeqId> for ControlTransition<SeqId>
where
    SeqId: SequenceId,
{
    fn control_transition(&self) -> &ControlTransition<SeqId> {
        self
    }
}
