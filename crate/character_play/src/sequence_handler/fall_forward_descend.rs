use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{common::SequenceRepeat, CharacterSequenceHandler, SwitchSequenceOnLand},
    CharacterSequenceUpdateComponents,
};

const FALL_FORWARD_DESCEND_BOUNCE: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceName::FallForwardLand);
const FALL_FORWARD_DESCEND_LIE: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceName::LieFaceDown);

#[derive(Debug)]
pub(crate) struct FallForwardDescend;

impl CharacterSequenceHandler for FallForwardDescend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.velocity[1] <= -10. {
            FALL_FORWARD_DESCEND_BOUNCE.update(components)
        } else {
            FALL_FORWARD_DESCEND_LIE.update(components)
        }
        .or_else(|| SequenceRepeat::update(components))
    }
}
