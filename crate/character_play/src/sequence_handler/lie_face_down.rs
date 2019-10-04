use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd},
    CharacterSequenceUpdateComponents,
};

const LIE_FACE_DOWN: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceName::Stand);

/// `LieFaceDown` sequence update.
#[derive(Debug)]
pub struct LieFaceDown;

impl CharacterSequenceHandler for LieFaceDown {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.health_points > 0 {
            LIE_FACE_DOWN.update(components.sequence_status)
        } else {
            None
        }
    }
}
