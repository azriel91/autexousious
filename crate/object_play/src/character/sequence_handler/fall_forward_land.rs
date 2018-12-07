use object_model::config::object::CharacterSequenceId;

use crate::character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd};
use crate::CharacterSequenceUpdateComponents;

const FALL_FORWARD_LAND: SwitchSequenceOnEnd =
    SwitchSequenceOnEnd(CharacterSequenceId::LieFaceDown);

#[derive(Debug)]
pub(crate) struct FallForwardLand;

impl CharacterSequenceHandler for FallForwardLand {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        FALL_FORWARD_LAND.update(components.sequence_status)
    }
}
