use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, Kinematics, ObjectStatus, ObjectStatusUpdate, RunCounter},
};

use character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd};

const LIE_FACE_DOWN: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceId::Stand);

#[derive(Debug)]
pub(crate) struct LieFaceDown;

impl CharacterSequenceHandler for LieFaceDown {
    fn update(
        _controller_input: &ControllerInput,
        character_status: &CharacterStatus,
        object_status: &ObjectStatus<CharacterSequenceId>,
        _kinematics: &Kinematics<f32>,
        _run_counter: RunCounter,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        if character_status.hp > 0 {
            LIE_FACE_DOWN.update(object_status)
        } else {
            ObjectStatusUpdate::default()
        }
    }
}
