use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd},
    CharacterSequenceUpdateComponents,
};

const FALL_FORWARD_LAND: SwitchSequenceOnEnd =
    SwitchSequenceOnEnd(CharacterSequenceName::LieFaceDown);

/// `FallForwardLand` sequence update.
#[derive(Debug)]
pub struct FallForwardLand;

impl CharacterSequenceHandler for FallForwardLand {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        FALL_FORWARD_LAND.update(components.sequence_status)
    }
}
