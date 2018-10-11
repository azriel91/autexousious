use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, CharacterStatusUpdate, Kinematics},
    loaded::Character,
};

use character::sequence_handler::{
    CharacterSequenceHandler, Jump, JumpAscend, JumpDescend, JumpDescendLand, JumpOff, Run,
    RunStop, Stand, StandAttack, Walk,
};

/// Defines behaviour for a character in game.
#[derive(Debug)]
pub struct CharacterSequenceUpdater;

impl CharacterSequenceUpdater {
    /// Handles behaviour transition (if any) based on input.
    ///
    /// # Parameters
    ///
    /// * `character`: Loaded character configuration.
    /// * `controller_input`: Controller input for the character.
    /// * `character_status`: Character specific status attributes.
    /// * `sequence_ended`: Whether the current sequence has ended.
    /// * `kinematics`: Kinematics of the character.
    pub fn update(
        character: &Character,
        controller_input: &ControllerInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let sequence_handler = match character_status.object_status.sequence_id {
            CharacterSequenceId::Stand => Stand::update,
            CharacterSequenceId::StandAttack => StandAttack::update,
            CharacterSequenceId::Walk => Walk::update,
            CharacterSequenceId::Run => Run::update,
            CharacterSequenceId::RunStop => RunStop::update,
            CharacterSequenceId::Jump => Jump::update,
            CharacterSequenceId::JumpOff => JumpOff::update,
            CharacterSequenceId::JumpAscend => JumpAscend::update,
            CharacterSequenceId::JumpDescend => JumpDescend::update,
            CharacterSequenceId::JumpDescendLand => JumpDescendLand::update,
        };

        let mut status_update = sequence_handler(controller_input, character_status, kinematics);

        // Check if it's at the end of the sequence before switching to next.
        if character_status.object_status.sequence_state == SequenceState::End {
            let current_sequence_id = &character_status.object_status.sequence_id;
            let current_sequence =
                &character.definition.object_definition.sequences[current_sequence_id];

            // `next` from configuration overrides the state handler transition.
            if current_sequence.next.is_some() {
                status_update.object_status.sequence_id = current_sequence.next;
            }
        }

        status_update

        // TODO: overrides based on sequence configuration
    }
}
