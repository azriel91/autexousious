use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, CharacterStatusUpdate, Kinematics},
};

use character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd};

const LIE_FACE_DOWN: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceId::Stand);

#[derive(Debug)]
pub(crate) struct LieFaceDown;

impl CharacterSequenceHandler for LieFaceDown {
    fn update(
        controller_input: &ControllerInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        if character_status.hp > 0 {
            LIE_FACE_DOWN.update(controller_input, character_status, kinematics)
        } else {
            CharacterStatusUpdate::default()
        }
    }
}
