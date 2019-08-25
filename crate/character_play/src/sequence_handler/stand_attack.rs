use character_model::config::CharacterSequenceName;

use crate::sequence_handler::{
    switch_sequence_on_end::SwitchSequenceOnEnd, CharacterSequenceHandler,
    CharacterSequenceUpdateComponents,
};

const STAND_ATTACK: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceName::Stand);

#[derive(Debug)]
pub(crate) struct StandAttack;

impl CharacterSequenceHandler for StandAttack {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        STAND_ATTACK.update(components.sequence_status)
    }
}
