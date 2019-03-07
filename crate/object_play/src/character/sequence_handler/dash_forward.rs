use character_model::config::CharacterSequenceId;

use crate::{
    character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEndYVelocity},
    CharacterSequenceUpdateComponents,
};

const SWITCH_SEQUENCE_ON_END_Y_VELOCITY: SwitchSequenceOnEndYVelocity =
    SwitchSequenceOnEndYVelocity {
        upwards: CharacterSequenceId::DashForwardAscend,
        downwards: CharacterSequenceId::DashForwardDescend,
    };

#[derive(Debug)]
pub(crate) struct DashForward;

impl CharacterSequenceHandler for DashForward {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        SWITCH_SEQUENCE_ON_END_Y_VELOCITY.update(components)
    }
}
