use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd},
    CharacterSequenceUpdateComponents,
};

const JUMP: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceName::JumpOff);

/// `Jump` sequence update.
#[derive(Debug)]
pub struct Jump;

impl CharacterSequenceHandler for Jump {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        JUMP.update(components.sequence_status)
    }
}
