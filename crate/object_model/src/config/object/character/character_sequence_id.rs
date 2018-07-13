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
    /// Character is about to jump.
    Jump,
    /// Character has just jumped off the ground.
    JumpOff,
    /// Character is moving upwards from jumping.
    ///
    /// This is distinct from the `Airborne` state as it is where the jump velocity is effective,
    /// and characters may have different animations and attacks when moving upwards from a jump.
    JumpAscend,
    /// Character is airborne.
    Airborne,
    /// Character landed from being airborne.
    AirborneLand,
}

impl SequenceId for CharacterSequenceId {}
