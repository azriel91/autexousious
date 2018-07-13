use config::object::SequenceId;

/// Object Sequence IDs.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, Eq, Hash, PartialEq)]
#[derivative(Default)]
#[serde(rename_all = "snake_case")]
pub enum CharacterSequenceId {
    /// Default sequence for characters.
    #[derivative(Default)]
    Stand,
    /// Walking sequence.
    Walk,
    /// Running sequence.
    Run,
    /// Stop running sequence.
    StopRun,
    /// Character is jumping.
    Jump,
    /// Character is airborne.
    Airborne,
    /// Character landed from being airborne.
    AirborneLand,
}

impl SequenceId for CharacterSequenceId {}
