use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, Kinematics, ObjectStatusUpdate},
};

use character::sequence_handler::SequenceHandler;

/// Determines whether to swithc to the `Walk` or `Run` sequence based on X input.
///
/// This should only be called from the Stand or Walk sequence handlers.
#[derive(Debug)]
pub(crate) struct SequenceRepeat;

impl SequenceHandler for SequenceRepeat {
    fn update(
        _input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> Option<ObjectStatusUpdate<CharacterSequenceId>> {
        if character_status.object_status.sequence_state == SequenceState::End {
            let sequence_id = Some(character_status.object_status.sequence_id);
            let sequence_state = Some(SequenceState::Begin);
            let mirrored = None;
            let grounding = None;

            Some(ObjectStatusUpdate::new(
                sequence_id,
                sequence_state,
                mirrored,
                grounding,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{CharacterInput, CharacterStatus, Kinematics, ObjectStatus, ObjectStatusUpdate},
    };

    use super::SequenceRepeat;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_change_when_sequence_begin() {
        assert_eq!(
            None,
            SequenceRepeat::update(
                &CharacterInput::default(),
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        sequence_state: SequenceState::Begin,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn no_change_when_sequence_ongoing() {
        assert_eq!(
            None,
            SequenceRepeat::update(
                &CharacterInput::default(),
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        sequence_state: SequenceState::Ongoing,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn restarts_sequence_when_no_input_and_sequence_end() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            }),
            SequenceRepeat::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        sequence_state: SequenceState::End,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }
}
