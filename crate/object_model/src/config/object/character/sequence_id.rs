/// Object Sequence IDs.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SequenceId {
    /// Default sequence for characters.
    Stand,
    /// Walking sequence.
    Walk,
}
