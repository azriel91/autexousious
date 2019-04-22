use sequence_model::config::SequenceId;

use crate::config::{GameObjectFrame, ObjectSequence};

/// Components common to object types' sequences, associated with a sequence ID.
pub trait GameObjectSequence {
    /// Sequence ID that this `GameObjectSequence` uses.
    type SequenceId: SequenceId;
    /// Type of the sequence frame.
    type GameObjectFrame: GameObjectFrame;

    /// Returns the `ObjectSequence` for this `GameObjectSequence`.
    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceId, Self::GameObjectFrame>;
}
