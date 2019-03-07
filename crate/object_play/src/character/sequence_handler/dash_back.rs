use character_model::config::CharacterSequenceId;

use crate::{
    character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEndYVelocity},
    CharacterSequenceUpdateComponents,
};

const SWITCH_SEQUENCE_ON_END_Y_VELOCITY: SwitchSequenceOnEndYVelocity =
    SwitchSequenceOnEndYVelocity {
        upwards: CharacterSequenceId::DashBackAscend,
        downwards: CharacterSequenceId::DashBackDescend,
    };

#[derive(Debug)]
pub(crate) struct DashBack;

impl CharacterSequenceHandler for DashBack {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        SWITCH_SEQUENCE_ON_END_Y_VELOCITY.update(components)
    }
}
