use object_model::{config::object::CharacterSequenceId, entity::ObjectStatusUpdate};

use character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd};
use CharacterSequenceUpdateComponents;

const FALL_FORWARD_LAND: SwitchSequenceOnEnd =
    SwitchSequenceOnEnd(CharacterSequenceId::LieFaceDown);

#[derive(Debug)]
pub(crate) struct FallForwardLand;

impl CharacterSequenceHandler for FallForwardLand {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        FALL_FORWARD_LAND.update(components.object_status)
    }
}
