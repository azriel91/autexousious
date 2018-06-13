use amethyst::{
    animation::{get_animation_set, AnimationCommand, AnimationControlSet, EndControl},
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
        animation_control_set_storage: &mut WriteStorage<
            's,
            AnimationControlSet<SequenceId, Material>,
        >,
        input: &CharacterInput,
        character: &Character,
        current_sequence_id: &SequenceId,
    ) -> Option<SequenceId> {
        let next_sequence = match *current_sequence_id {
            SequenceId::Stand => sequence_handler::Stand::update(input),
            SequenceId::Walk => sequence_handler::Walk::update(input),
        };

        if let Some(ref next_sequence) = next_sequence {
            let animation_handle = &character
                .object
                .animations
                .get(next_sequence)
                .unwrap_or_else(|| {
                    panic!(
                        "Failed to get animation for sequence: `{:?}`",
                        next_sequence
                    )
                })
                .clone();

            let animation_set =
                get_animation_set::<SequenceId, Material>(animation_control_set_storage, *entity);

            // Abort the previous animation
            animation_set.abort(*current_sequence_id);

            // Start the next animation
            animation_set.add_animation(
                *next_sequence,
                &animation_handle,
                EndControl::Loop(None),
                30., // Rate at which the animation plays
                AnimationCommand::Start,
            );
        }

        next_sequence
    }
}
