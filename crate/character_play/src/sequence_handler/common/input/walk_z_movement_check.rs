use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{common::SequenceRepeat, CharacterSequenceHandler},
    CharacterSequenceUpdateComponents,
};

/// Determines whether to switch to the `Stand` sequence based on Z input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub struct WalkZMovementCheck;

impl CharacterSequenceHandler for WalkZMovementCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.controller_input.z_axis_value != 0. {
            SequenceRepeat::update(components)
        } else {
            None
        }
    }
}
