use character_model::config::CharacterSequenceName;

use crate::sequence_handler::{
    switch_sequence_on_end::SwitchSequenceOnEnd, CharacterSequenceHandler,
    CharacterSequenceUpdateComponents,
};

const JUMP_DESCEND_LAND: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceName::Stand);

/// `JumpDescendLand` sequence update.
#[derive(Debug)]
pub struct JumpDescendLand;

impl CharacterSequenceHandler for JumpDescendLand {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        JUMP_DESCEND_LAND.update(components.sequence_status)
    }
}
