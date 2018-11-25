use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, Kinematics, ObjectStatus, ObjectStatusUpdate, RunCounter},
};

use character::sequence_handler::{CharacterSequenceHandler, SwitchSequenceOnEnd};

const STAND_ON_SEQUENCE_END: SwitchSequenceOnEnd = SwitchSequenceOnEnd(CharacterSequenceId::Stand);

#[derive(Debug)]
pub(crate) struct StandOnSequenceEnd;

impl CharacterSequenceHandler for StandOnSequenceEnd {
    fn update(
        _controller_input: &ControllerInput,
        _character_status: &CharacterStatus,
        object_status: &ObjectStatus<CharacterSequenceId>,
        _kinematics: &Kinematics<f32>,
        _run_counter: RunCounter,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        STAND_ON_SEQUENCE_END.update(object_status)
    }
}
