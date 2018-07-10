use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate},
    loaded::Character,
};

use character::sequence_handler::{Run, SequenceHandler, Stand, Walk};

/// Defines behaviour for a character in game.
#[derive(Debug)]
pub struct CharacterSequenceHandler;

impl CharacterSequenceHandler {
    /// Handles behaviour transition (if any) based on input.
    pub fn update(
        _character: &Character,
        input: &CharacterInput,
        character_status: &CharacterStatus,
    ) -> CharacterStatusUpdate {
        let sequence_handler = match character_status.object_status.sequence_id {
            CharacterSequenceId::Stand => Stand::update,
            CharacterSequenceId::Walk => Walk::update,
            CharacterSequenceId::Run => Run::update,
        };

        sequence_handler(input, character_status)

        // TODO: overrides based on sequence configuration
    }
}
