use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd},
    CharacterSequenceUpdateComponents,
};

const STAND_ON_SEQUENCE_END: SwitchSequenceOnEnd =
    SwitchSequenceOnEnd(CharacterSequenceName::Stand);

#[derive(Debug)]
pub(crate) struct StandOnSequenceEnd;

impl CharacterSequenceHandler for StandOnSequenceEnd {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        STAND_ON_SEQUENCE_END.update(components.sequence_status)
    }
}
