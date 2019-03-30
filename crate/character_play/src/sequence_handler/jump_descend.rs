use character_model::config::CharacterSequenceId;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnLand},
    CharacterSequenceUpdateComponents,
};

const JUMP_DESCEND_LAND: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceId::JumpDescendLand);

#[derive(Debug)]
pub(crate) struct JumpDescend;

impl CharacterSequenceHandler for JumpDescend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        JUMP_DESCEND_LAND.update(components)
    }
}
