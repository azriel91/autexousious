// See comment on struct. This attribute isn't detected if we put it on the
// struct.
#![allow(clippy::too_many_arguments)]

use character_model::{config::CharacterSequenceName, play::RunCounter};
use derive_new::new;
use game_input_model::play::ControllerInput;
use kinematic_model::config::{Position, Velocity};
use mirrored_model::play::Mirrored;
use object_model::play::{Grounding, HealthPoints};
use sequence_model::play::SequenceStatus;

/// Components used to compute character sequence updates.
///
/// TODO: Reduce number of arguments passed, perhaps by splitting the sequence
/// update handlers into separate systems.
#[derive(Clone, Copy, Debug, new)]
pub struct CharacterSequenceUpdateComponents<'c> {
    /// Controller input of the character.
    pub controller_input: &'c ControllerInput,
    /// Health points.
    pub health_points: HealthPoints,
    /// Current character sequence name.
    pub character_sequence_name: CharacterSequenceName,
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
    /// States used to track X axis input over time to determine when a
    /// character should run.
    pub run_counter: RunCounter,
}
