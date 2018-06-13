use amethyst::{
    animation::AnimationControlSet,
    ecs::{prelude::*, storage::WriteStorage},
    renderer::Material,
};
use object_model::{
    config::object::character::SequenceId, entity::CharacterInput, loaded::Character,
};

use character::sequence_handler::{self, SequenceHandler};

/// Defines behaviour for a character in game.
#[derive(Debug)]
pub struct CharacterSequenceHandler;

impl CharacterSequenceHandler {
    /// Handles behaviour transition (if any) based on input.
    pub fn update<'s>(
        entity: &Entity,
        animation_control_set_storage: &mut WriteStorage<'s, AnimationControlSet<u32, Material>>,
        input: &CharacterInput,
        character: &Character,
        current_sequence_id: &SequenceId,
    ) -> Option<SequenceId> {
        let next_sequence = match *current_sequence_id {
            SequenceId::Stand => sequence_handler::Stand::update(input),
            SequenceId::Walk => sequence_handler::Walk::update(input),
        };

        if let Some(ref next_sequence) = next_sequence {
            match next_sequence {
                SequenceId::Stand => {
                    sequence_handler::Stand::begin(entity, character, animation_control_set_storage)
                }
                SequenceId::Walk => {
                    sequence_handler::Walk::begin(entity, character, animation_control_set_storage)
                }
            }
        }

        next_sequence
    }
}
