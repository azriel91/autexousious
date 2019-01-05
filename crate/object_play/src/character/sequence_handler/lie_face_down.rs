use character_model::config::CharacterSequenceId;

use crate::{
    character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd},
    CharacterSequenceUpdateComponents,
};

const LIE_FACE_DOWN: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceId::Stand);

#[derive(Debug)]
pub(crate) struct LieFaceDown;

impl CharacterSequenceHandler for LieFaceDown {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        if components.health_points > 0 {
            LIE_FACE_DOWN.update(components.sequence_status)
        } else {
            None
        }
    }
}
