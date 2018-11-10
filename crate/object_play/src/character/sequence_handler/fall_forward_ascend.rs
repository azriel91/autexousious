use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, CharacterStatusUpdate, Kinematics},
};

use character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnDescend};

const FALL_FORWARD_ASCEND: SwitchSequenceOnDescend =
    SwitchSequenceOnDescend(CharacterSequenceId::FallForwardDescend);

#[derive(Debug)]
pub(crate) struct FallForwardAscend;

impl CharacterSequenceHandler for FallForwardAscend {
    fn update(
        controller_input: &ControllerInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        FALL_FORWARD_ASCEND.update(controller_input, character_status, kinematics)
    }
}
