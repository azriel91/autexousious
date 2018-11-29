use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{
        CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, RunCounter, SequenceStatus,
    },
};

/// Components used to compute character sequence updates.
#[derive(Clone, Copy, Debug, new)]
pub struct CharacterSequenceUpdateComponents<'c> {
    /// Controller input of the character.
    pub controller_input: &'c ControllerInput,
    /// Character-specific status attributes.
    pub character_status: &'c CharacterStatus,
    /// Status of an object entity.
    pub object_status: &'c ObjectStatus<CharacterSequenceId>,
    /// Whether a sequence has just begun, is ongoing, or has ended.
    pub sequence_status: SequenceStatus,
    /// Grouping of motion attributes.
    pub kinematics: &'c Kinematics<f32>,
    /// Whether or not this object is facing left.
    pub mirrored: Mirrored,
    /// Tracks an object's attachment to the surrounding environment.
    pub grounding: Grounding,
    /// States used to track X axis input over time to determine when a character should run.
    pub run_counter: RunCounter,
}
