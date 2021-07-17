use character_model::{config::CharacterSequenceName, play::RunCounter};

use crate::{
    sequence_handler::{common::SequenceRepeat, CharacterSequenceHandler, SequenceHandlerUtil},
    CharacterSequenceUpdateComponents,
};

/// Determines whether to switch to the `Walk` or `Run` sequence based on X
/// input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub struct WalkXMovementCheck;

impl CharacterSequenceHandler for WalkXMovementCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.controller_input.x_axis_value != 0. {
            let same_direction = SequenceHandlerUtil::input_matches_direction(
                components.controller_input,
                components.mirrored,
            );

            let sequence_id = match (components.run_counter, same_direction) {
                (RunCounter::Unused, _) | (RunCounter::Increase(_), false) => {
                    Some(CharacterSequenceName::Walk)
                }
                (RunCounter::Decrease(_), true) => Some(CharacterSequenceName::Run),
                (RunCounter::Exceeded, _)
                | (RunCounter::Decrease(_), false)
                | (RunCounter::Increase(_), true) => None,
            };

            if sequence_id.is_none() {
                SequenceRepeat::update(components)
            } else {
                sequence_id
            }
        } else {
            // The responsibility of switching to `Stand` is handled elsewhere.
            None
        }
    }
}
