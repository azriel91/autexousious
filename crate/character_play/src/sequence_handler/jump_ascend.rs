use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnDescend},
    CharacterSequenceUpdateComponents,
};

const JUMP_ASCEND: SwitchSequenceOnDescend =
    SwitchSequenceOnDescend(CharacterSequenceName::JumpDescend);

/// `JumpAscend` sequence update.
#[derive(Debug)]
pub struct JumpAscend;

impl CharacterSequenceHandler for JumpAscend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        JUMP_ASCEND.update(components)
    }
}
