use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate},
    loaded::Character,
};

use character::sequence_handler::{Run, SequenceHandler, Stand, StopRun, Walk};

/// Defines behaviour for a character in game.
#[derive(Debug)]
pub struct CharacterSequenceHandler;

impl CharacterSequenceHandler {
    /// Handles behaviour transition (if any) based on input.
    ///
    /// # Parameters
    ///
    /// * `character`: Loaded character configuration.
    /// * `character_input`: Controller input for the character.
    /// * `character_status`: Character specific status attributes.
    /// * `sequence_ended`: Whether the current sequence has ended.
    pub fn update(
        character: &Character,
        character_input: &CharacterInput,
        character_status: &CharacterStatus,
        sequence_ended: bool,
    ) -> CharacterStatusUpdate {
        let sequence_handler = match character_status.object_status.sequence_id {
            CharacterSequenceId::Stand => Stand::update,
            CharacterSequenceId::Walk => Walk::update,
            CharacterSequenceId::Run => Run::update,
            CharacterSequenceId::StopRun => StopRun::update,
        };

        // Pass sequence_ended through to sequence handlers, which lets them decide to loop the
        // sequence when it has finished.
        let mut status_update = sequence_handler(character_input, character_status, sequence_ended);

        // Check if it's at the end of the sequence before switching to next.
        if sequence_ended {
            let current_sequence_id = &character_status.object_status.sequence_id;
            let current_sequence = character
                .definition
                .object_definition
                .sequences
                .get(current_sequence_id)
                .unwrap();

            // `next` from configuration overrides the state handler transition.
            if current_sequence.next.is_some() {
                status_update.object_status.sequence_id = current_sequence.next.clone();
            }
        }

        status_update

        // TODO: overrides based on sequence configuration
    }
}
