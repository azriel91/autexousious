use object_model::{config::object::CharacterSequenceId, entity::ObjectStatusUpdate};

use character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd};
use CharacterSequenceUpdateComponents;

const STAND_ON_SEQUENCE_END: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceId::Stand);

#[derive(Debug)]
pub(crate) struct StandOnSequenceEnd;

impl CharacterSequenceHandler for StandOnSequenceEnd {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        STAND_ON_SEQUENCE_END.update(components.object_status)
    }
}
