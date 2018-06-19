use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterInput, ObjectStatusUpdate},
    loaded::Character,
};

use character::sequence_handler::{self, SequenceHandler};

/// Defines behaviour for a character in game.
#[derive(Debug)]
pub struct CharacterSequenceHandler;

impl CharacterSequenceHandler {
    /// Handles behaviour transition (if any) based on input.
    pub fn update(
        _character: &Character,
        input: &CharacterInput,
        current_sequence_id: &CharacterSequenceId,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        match *current_sequence_id {
            CharacterSequenceId::Stand => sequence_handler::Stand::update(input),
            CharacterSequenceId::Walk => sequence_handler::Walk::update(input),
        }

        // TODO: overrides based on sequence configuration
    }
}
