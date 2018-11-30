use object_model::config::object::CharacterSequenceId;

use character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnLand};
use CharacterSequenceUpdateComponents;

const FALL_FORWARD_DESCEND_BOUNCE: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceId::FallForwardLand);
const FALL_FORWARD_DESCEND_LIE: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceId::LieFaceDown);

#[derive(Debug)]
pub(crate) struct FallForwardDescend;

impl CharacterSequenceHandler for FallForwardDescend {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        if components.kinematics.velocity[1] <= -10. {
            FALL_FORWARD_DESCEND_BOUNCE.update(components)
        } else {
            FALL_FORWARD_DESCEND_LIE.update(components)
        }
    }
}
