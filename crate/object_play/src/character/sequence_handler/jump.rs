use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
        RunCounter,
    },
};

use character::sequence_handler::CharacterSequenceHandler;

#[derive(Debug)]
pub(crate) struct Jump;

impl CharacterSequenceHandler for Jump {
    fn update(
        _controller_input: &ControllerInput,
        _character_status: &CharacterStatus,
        object_status: &ObjectStatus<CharacterSequenceId>,
        _kinematics: &Kinematics<f32>,
        _run_counter: RunCounter,
    ) -> (
        CharacterStatusUpdate,
        ObjectStatusUpdate<CharacterSequenceId>,
    ) {
        let character_status_update = CharacterStatusUpdate::default();
        let mut object_status_update = ObjectStatusUpdate::default();
        if object_status.sequence_state == SequenceState::End {
            object_status_update.sequence_id = Some(CharacterSequenceId::JumpOff);
            object_status_update.sequence_state = Some(SequenceState::Begin);
        }

        (character_status_update, object_status_update)
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
            RunCounter,
        },
    };

    use super::Jump;
    use character::sequence_handler::CharacterSequenceHandler;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::default()
            ),
            Jump::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Jump,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn switches_to_jump_off_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpOff),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            Jump::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Jump,
                    sequence_state: SequenceState::End,
                    ..Default::default()
                },
                &kinematics,
                RunCounter::default()
            )
        );
    }
}
