use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEndYVelocity},
    CharacterSequenceUpdateComponents,
};

const SWITCH_SEQUENCE_ON_END_Y_VELOCITY: SwitchSequenceOnEndYVelocity =
    SwitchSequenceOnEndYVelocity {
        upwards: CharacterSequenceName::JumpAscend,
        downwards: CharacterSequenceName::JumpDescend,
    };

/// `JumpOff` sequence update.
#[derive(Debug)]
pub struct JumpOff;

impl CharacterSequenceHandler for JumpOff {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        SWITCH_SEQUENCE_ON_END_Y_VELOCITY.update(components)
    }
}
