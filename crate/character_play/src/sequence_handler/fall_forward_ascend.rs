use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnDescend},
    CharacterSequenceUpdateComponents,
};

const FALL_FORWARD_ASCEND: SwitchSequenceOnDescend =
    SwitchSequenceOnDescend(CharacterSequenceName::FallForwardDescend);

/// `FallForwardAscend` sequence update.
#[derive(Debug)]
pub struct FallForwardAscend;

impl CharacterSequenceHandler for FallForwardAscend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        FALL_FORWARD_ASCEND.update(components)
    }
}
