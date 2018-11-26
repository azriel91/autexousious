use object_model::{config::object::CharacterSequenceId, entity::ObjectStatusUpdate};

use character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnDescend};
use CharacterSequenceUpdateComponents;

const FALL_FORWARD_ASCEND: SwitchSequenceOnDescend =
    SwitchSequenceOnDescend(CharacterSequenceId::FallForwardDescend);

#[derive(Debug)]
pub(crate) struct FallForwardAscend;

impl CharacterSequenceHandler for FallForwardAscend {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        FALL_FORWARD_ASCEND.update(components)
    }
}
