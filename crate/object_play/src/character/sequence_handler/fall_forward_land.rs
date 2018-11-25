use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, Kinematics, ObjectStatus, ObjectStatusUpdate, RunCounter},
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
        object_status: &ObjectStatus<CharacterSequenceId>,
        kinematics: &Kinematics<f32>,
        run_counter: RunCounter,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        FALL_FORWARD_LAND.update(
            controller_input,
            character_status,
            object_status,
            kinematics,
            run_counter,
        )
    }
}
