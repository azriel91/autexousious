use character_model::config::{CharacterSequenceName, CharacterSequenceNameString};
use game_input::ControllerInput;
use mirrored_model::play::Mirrored;
use sequence_model::config::SequenceNameString;

use crate::sequence_handler::SequenceHandlerUtil;

/// Updates the `Mirrored` component for character entities.
#[derive(Debug)]
pub struct MirroredUpdater;

impl MirroredUpdater {
    /// Returns the updated `Mirrored` value.
    ///
    /// # Parameters
    ///
    /// * `controller_input`: Controller input for this character.
    /// * `character_sequence_name_string`: Current character sequence name.
    /// * `mirrored`: Whether the object is mirrored (facing left).
    pub fn update(
        controller_input: &ControllerInput,
        character_sequence_name_string: &CharacterSequenceNameString,
        mirrored: Mirrored,
    ) -> Mirrored {
        match character_sequence_name_string {
            SequenceNameString::Name(CharacterSequenceName::Stand)
            | SequenceNameString::Name(CharacterSequenceName::Walk)
            | SequenceNameString::Name(CharacterSequenceName::JumpAscend)
            | SequenceNameString::Name(CharacterSequenceName::JumpDescend) => {}
            _ => return mirrored,
        }

        if SequenceHandlerUtil::input_opposes_direction(controller_input, mirrored) {
            !mirrored
        } else {
            mirrored
        }
    }
}
