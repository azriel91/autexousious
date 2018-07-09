use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterInput, CharacterStatus, ObjectStatusUpdate},
    loaded::Character,
};

use character::sequence_handler::{SequenceHandler, Stand, Walk};

/// Defines behaviour for a character in game.
#[derive(Debug)]
pub struct CharacterSequenceHandler;

impl CharacterSequenceHandler {
    /// Handles behaviour transition (if any) based on input.
    pub fn update(
        _character: &Character,
        input: &CharacterInput,
        current_sequence_id: &CharacterSequenceId,
        character_status: &mut CharacterStatus,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        let sequence_handler = match *current_sequence_id {
            CharacterSequenceId::Stand => Stand::update,
            CharacterSequenceId::Walk => Walk::update,
        };

        sequence_handler(input, character_status)

        // TODO: overrides based on sequence configuration
    }
}
