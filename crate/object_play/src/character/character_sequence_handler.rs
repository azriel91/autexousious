use object_model::{
    config::object::character::SequenceId,
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
        current_sequence_id: &SequenceId,
    ) -> ObjectStatusUpdate<SequenceId> {
        match *current_sequence_id {
            SequenceId::Stand => sequence_handler::Stand::update(input),
            SequenceId::Walk => sequence_handler::Walk::update(input),
        }

        // TODO: overrides based on sequence configuration
    }
}
