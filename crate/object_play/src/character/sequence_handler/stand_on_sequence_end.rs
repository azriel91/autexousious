use object_model::config::object::CharacterSequenceId;

use crate::{
    character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd},
    CharacterSequenceUpdateComponents,
};

const STAND_ON_SEQUENCE_END: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceId::Stand);

#[derive(Debug)]
pub(crate) struct StandOnSequenceEnd;

impl CharacterSequenceHandler for StandOnSequenceEnd {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        STAND_ON_SEQUENCE_END.update(components.sequence_status)
    }
}
