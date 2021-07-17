use character_model::{config::CharacterSequenceName, play::RunCounter};

use crate::{
    sequence_handler::{CharacterSequenceHandler, SequenceHandlerUtil},
    CharacterSequenceUpdateComponents,
};

/// Determines whether to swithc to the `Walk` or `Run` sequence based on X
/// input.
///
/// This should only be called from the Stand sequence handler.
#[derive(Debug)]
pub struct StandXMovementCheck;

impl CharacterSequenceHandler for StandXMovementCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.controller_input.x_axis_value != 0. {
            let same_direction = SequenceHandlerUtil::input_matches_direction(
                components.controller_input,
                components.mirrored,
            );

            match components.run_counter {
                RunCounter::Unused => Some(CharacterSequenceName::Walk),
                RunCounter::Decrease(_) => {
                    if same_direction {
                        Some(CharacterSequenceName::Run)
                    } else {
                        Some(CharacterSequenceName::Walk)
                    }
                }
                _ => unreachable!(), // kcov-ignore
            }
        } else {
            None
        }
    }
}
