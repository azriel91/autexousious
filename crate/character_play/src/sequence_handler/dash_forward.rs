use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEndYVelocity},
    CharacterSequenceUpdateComponents,
};

const SWITCH_SEQUENCE_ON_END_Y_VELOCITY: SwitchSequenceOnEndYVelocity =
    SwitchSequenceOnEndYVelocity {
        upwards: CharacterSequenceName::DashForwardAscend,
        downwards: CharacterSequenceName::DashForwardDescend,
    };

#[derive(Debug)]
pub(crate) struct DashForward;

impl CharacterSequenceHandler for DashForward {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        SWITCH_SEQUENCE_ON_END_Y_VELOCITY.update(components)
    }
}
