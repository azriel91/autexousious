use character_model::config::CharacterSequenceName;
use object_model::play::Grounding;

use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

/// Returns a `JumpDescend` update if the grounding is `Airborne`.
#[derive(Debug)]
pub struct AirborneCheck;

impl CharacterSequenceHandler for AirborneCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.grounding == Grounding::Airborne {
            Some(CharacterSequenceName::JumpDescend)
        } else {
            None
        }
    }
}
