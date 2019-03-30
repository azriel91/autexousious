use character_model::config::CharacterSequenceId;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEndYVelocity},
    CharacterSequenceUpdateComponents,
};

const SWITCH_SEQUENCE_ON_END_Y_VELOCITY: SwitchSequenceOnEndYVelocity =
    SwitchSequenceOnEndYVelocity {
        upwards: CharacterSequenceId::JumpAscend,
        downwards: CharacterSequenceId::JumpDescend,
    };

#[derive(Debug)]
pub(crate) struct JumpOff;

impl CharacterSequenceHandler for JumpOff {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        SWITCH_SEQUENCE_ON_END_Y_VELOCITY.update(components)
    }
}
