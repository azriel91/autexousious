use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, CharacterStatusUpdate, Kinematics},
};

use character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd};

const FALL_FORWARD_LAND: SwitchSequenceOnEnd =
    SwitchSequenceOnEnd(CharacterSequenceId::LieFaceDown);

#[derive(Debug)]
pub(crate) struct FallForwardLand;

impl CharacterSequenceHandler for FallForwardLand {
    fn update(
        controller_input: &ControllerInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        FALL_FORWARD_LAND.update(controller_input, character_status, kinematics)
    }
}
