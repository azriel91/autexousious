use character_model::{
    config::{CharacterSequenceName, CharacterSequenceNameString},
    play::RunCounter,
};
use game_input_model::play::ControllerInput;
use mirrored_model::play::Mirrored;
use object_model::play::Grounding;
use sequence_model::config::SequenceNameString;

use crate::sequence_handler::SequenceHandlerUtil;

/// Updates the `RunCounter` component for character entities.
#[derive(Debug)]
pub struct RunCounterUpdater;

impl RunCounterUpdater {
    /// Returns the updated `RunCounter` value.
    ///
    /// # Parameters
    ///
    /// * `run_counter`: Original `RunCounter` value.
    /// * `controller_input`: Controller input for this character.
    /// * `character_sequence_name`: Current character sequence name.
    /// * `mirrored`: Whether the object is mirrored (facing left).
    /// * `grounding`: Whether the object is on the ground.
    pub fn update(
        run_counter: RunCounter,
        controller_input: &ControllerInput,
        character_sequence_name_string: &CharacterSequenceNameString,
        mirrored: Mirrored,
        grounding: Grounding,
    ) -> RunCounter {
        match character_sequence_name_string {
            SequenceNameString::Name(CharacterSequenceName::Stand)
            | SequenceNameString::Name(CharacterSequenceName::Walk) => {}
            _ => return RunCounter::Unused,
        }

        if grounding != Grounding::OnGround {
            return RunCounter::Unused;
        }

        use character_model::play::RunCounter::*;
        if controller_input.x_axis_value == 0. {
            match run_counter {
                Unused => run_counter,
                Exceeded | Decrease(0) => RunCounter::Unused,
                Decrease(ticks) => Decrease(ticks - 1),
                Increase(_) => Decrease(RunCounter::RESET_TICK_COUNT),
            }
        } else {
            let same_direction =
                SequenceHandlerUtil::input_matches_direction(controller_input, mirrored);
            match (run_counter, same_direction) {
                (Unused, _) | (Decrease(_), false) | (Increase(_), false) => {
                    Increase(RunCounter::RESET_TICK_COUNT)
                }
                (Decrease(_), true) => Unused, // Switch to running
                (Increase(0), true) => Exceeded,
                (Increase(ticks), true) => Increase(ticks - 1),
                (Exceeded, _) => run_counter,
            }
        }
    }
}
