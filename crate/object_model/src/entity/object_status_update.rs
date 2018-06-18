use std::hash::Hash;

/// Indicates what fields of an `ObjectStatus` should be updated.
// TODO: Learning exercise - Generate this using a proc macro
#[derive(Constructor, Default, Debug, PartialEq)]
pub struct ObjectStatusUpdate<SeqId: Copy + Eq + Hash + Send + Sync> {
    /// ID of the current sequence the entity is on.
    pub sequence_id: Option<SeqId>,
    /// Whether or not this object is facing left.
    pub mirrored: Option<bool>,
}
