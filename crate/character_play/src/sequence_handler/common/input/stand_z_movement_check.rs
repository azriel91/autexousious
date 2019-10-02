use character_model::config::CharacterSequenceName;

use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

/// Determines whether to switch to the `Walk` sequence based on Z input.
///
/// This should only be called from the Stand sequence handler.
#[derive(Debug)]
pub struct StandZMovementCheck;

impl CharacterSequenceHandler for StandZMovementCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.controller_input.z_axis_value != 0. {
            Some(CharacterSequenceName::Walk)
        } else {
            None
        }
    }
}
