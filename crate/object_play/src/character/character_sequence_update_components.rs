use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{
        CharacterStatus, Grounding, Mirrored, Position, RunCounter, SequenceStatus, Velocity,
    },
};

/// Components used to compute character sequence updates.
#[derive(Clone, Copy, Debug, new)]
pub struct CharacterSequenceUpdateComponents<'c> {
    /// Controller input of the character.
    pub controller_input: &'c ControllerInput,
    /// Character-specific status attributes.
    pub character_status: &'c CharacterStatus,
    /// Current character sequence ID.
    pub character_sequence_id: CharacterSequenceId,
    /// Whether a sequence has just begun, is ongoing, or has ended.
    pub sequence_status: SequenceStatus,
    /// Position of the character.
    pub position: &'c Position<f32>,
    /// Velocity of the character.
    pub velocity: &'c Velocity<f32>,
    /// Whether or not this object is facing left.
    pub mirrored: Mirrored,
    /// Tracks an object's attachment to the surrounding environment.
    pub grounding: Grounding,
    /// States used to track X axis input over time to determine when a character should run.
    pub run_counter: RunCounter,
}
