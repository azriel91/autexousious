use character_model::config::CharacterSequenceId;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnLand},
    CharacterSequenceUpdateComponents,
};

const FALL_FORWARD_DESCEND_BOUNCE: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand);
const FALL_FORWARD_DESCEND_LIE: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceId::LieFaceDown);

#[derive(Debug)]
pub(crate) struct FallForwardDescend;

impl CharacterSequenceHandler for FallForwardDescend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        if components.velocity[1] <= -10. {
            FALL_FORWARD_DESCEND_BOUNCE.update(components)
        } else {
            FALL_FORWARD_DESCEND_LIE.update(components)
        }
    }
}
