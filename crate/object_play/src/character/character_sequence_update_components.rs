use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, Kinematics, ObjectStatus, RunCounter},
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
    /// Grouping of motion attributes.
    pub kinematics: &'c Kinematics<f32>,
    /// States used to track X axis input over time to determine when a character should run.
    pub run_counter: RunCounter,
}
