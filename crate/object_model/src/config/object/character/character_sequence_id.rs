use config::object::SequenceId;

/// Object Sequence IDs.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CharacterSequenceId {
    /// Default sequence for characters.
    Stand,
    /// Walking sequence.
    Walk,
}

impl Default for CharacterSequenceId {
    fn default() -> Self {
        CharacterSequenceId::Stand
    }
}

impl SequenceId for CharacterSequenceId {}
