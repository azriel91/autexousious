use sequence_model::config::{SequenceName, Wait};

use crate::config::{GameObjectFrame, ObjectSequence};

/// Components common to object types' sequences, associated with a sequence ID.
pub trait GameObjectSequence {
    /// Sequence ID that this `GameObjectSequence` uses.
    type SequenceName: SequenceName;
    /// Type of the sequence frame.
    type GameObjectFrame: AsRef<Wait> + Default + GameObjectFrame;

    /// Returns the `ObjectSequence` for this `GameObjectSequence`.
    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceName, Self::GameObjectFrame>;
}
