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
    pub fn update(
        character: &Character,
        input: &CharacterInput,
        character_status: &CharacterStatus,
        sequence_ended: bool,
    ) -> CharacterStatusUpdate {
        let sequence_handler = match character_status.object_status.sequence_id {
            CharacterSequenceId::Stand => Stand::update,
            CharacterSequenceId::Walk => Walk::update,
            CharacterSequenceId::Run => Run::update,
            CharacterSequenceId::StopRun => StopRun::update,
        };

        // TODO: pass sequence_ended through to sequence handlers, which lets them decide to loop
        // the sequence when it has finished.
        let mut status_update = sequence_handler(input, character_status);

        // Need to also check if it's at the end of the sequence before switching to next
        if sequence_ended && status_update.object_status.sequence_id.is_none() {
            let current_sequence_id = &character_status.object_status.sequence_id;
            let current_sequence = character
                .definition
                .object_definition
                .sequences
                .get(current_sequence_id)
                .unwrap();
            status_update.object_status.sequence_id = current_sequence.next.clone();
        }

        status_update

        // TODO: overrides based on sequence configuration
    }
}
