use config::object::{SequenceId, SequenceState};
use entity::{Grounding, Mirrored};

/// Indicates what fields of an `ObjectStatus` should be updated.
// TODO: Learning exercise - Generate this using a proc macro
// See <https://crates.io/crates/optional_struct>
#[derive(Default, Debug, PartialEq, new)]
pub struct ObjectStatusUpdate<SeqId: SequenceId> {
    /// ID of the current sequence the entity is on.
    pub sequence_id: Option<SeqId>,
    /// Whether the sequence just started, is ongoing, or has ended.
    pub sequence_state: Option<SequenceState>,
    /// Whether or not this object is facing left.
    pub mirrored: Option<Mirrored>,
    /// Tracks an object's attachment to the surrounding environment.
    pub grounding: Option<Grounding>,
}
