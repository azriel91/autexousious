/// Object Sequence IDs.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum SequenceId {
    /// Default sequence for characters.
    Standing,
    /// Walking sequence.
    Walking,
}
