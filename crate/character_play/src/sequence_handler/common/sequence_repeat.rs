use character_model::config::CharacterSequenceName;
use sequence_model::play::SequenceStatus;

use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

/// Restarts a sequence when it has reached the end.
#[derive(Debug)]
pub struct SequenceRepeat;

impl CharacterSequenceHandler for SequenceRepeat {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.sequence_status == SequenceStatus::End {
            Some(components.character_sequence_name)
        } else {
            None
        }
    }
}
