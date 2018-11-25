use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
        RunCounter,
    },
};

use character::sequence_handler::SequenceHandler;

/// Restarts a sequence when it has reached the end.
#[derive(Debug)]
pub(crate) struct SequenceRepeat;

impl SequenceHandler for SequenceRepeat {
    fn update(
        _input: &ControllerInput,
        _character_status: &CharacterStatus,
        object_status: &ObjectStatus<CharacterSequenceId>,
        _kinematics: &Kinematics<f32>,
        _run_counter: RunCounter,
    ) -> Option<(
        CharacterStatusUpdate,
        ObjectStatusUpdate<CharacterSequenceId>,
    )> {
        if object_status.sequence_state == SequenceState::End {
            let sequence_id = Some(object_status.sequence_id);
            let sequence_state = Some(SequenceState::Begin);
            let mirrored = None;
            let grounding = None;

            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::new(sequence_id, sequence_state, mirrored, grounding),
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
            RunCounter,
        },
    };

    use super::SequenceRepeat;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_change_when_sequence_begin() {
        assert_eq!(
            None,
            SequenceRepeat::update(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    sequence_state: SequenceState::Begin,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn no_change_when_sequence_ongoing() {
        assert_eq!(
            None,
            SequenceRepeat::update(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    sequence_state: SequenceState::Ongoing,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn restarts_sequence_when_no_input_and_sequence_end() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            )),
            SequenceRepeat::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    sequence_state: SequenceState::End,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }
}
