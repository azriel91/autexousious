use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{common::SequenceRepeat, CharacterSequenceHandler, SwitchSequenceOnLand},
    CharacterSequenceUpdateComponents,
};

const JUMP_DESCEND_LAND: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceName::JumpDescendLand);

/// `JumpDescend` sequence update.
#[derive(Debug)]
pub struct JumpDescend;

impl CharacterSequenceHandler for JumpDescend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        JUMP_DESCEND_LAND
            .update(components)
            .or_else(|| SequenceRepeat::update(components))
    }
}
