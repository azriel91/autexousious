use object_model::{config::object::CharacterSequenceId, entity::ObjectStatusUpdate};

use character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd};
use CharacterSequenceUpdateComponents;

const LIE_FACE_DOWN: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceId::Stand);

#[derive(Debug)]
pub(crate) struct LieFaceDown;

impl CharacterSequenceHandler for LieFaceDown {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        if components.character_status.hp > 0 {
            LIE_FACE_DOWN.update(components.object_status)
        } else {
            ObjectStatusUpdate::default()
        }
    }
}
