use character_model::config::CharacterSequenceName;

use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

/// Returns the appropriate falling sequence if HP is 0.
#[derive(Debug)]
pub struct AliveCheck;

impl CharacterSequenceHandler for AliveCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.health_points == 0 {
            Some(CharacterSequenceName::FallForwardDescend)
        } else {
            None
        }
    }
}
