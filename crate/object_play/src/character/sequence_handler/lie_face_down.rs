use object_model::config::object::CharacterSequenceId;

use character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd};
use CharacterSequenceUpdateComponents;

const LIE_FACE_DOWN: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceId::Stand);

#[derive(Debug)]
pub(crate) struct LieFaceDown;

impl CharacterSequenceHandler for LieFaceDown {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        if components.character_status.hp > 0 {
            LIE_FACE_DOWN.update(components.sequence_status)
        } else {
            None
        }
    }
}
