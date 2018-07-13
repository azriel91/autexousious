use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics},
};

use character::sequence_handler::SequenceHandler;

#[derive(Debug)]
pub(crate) struct JumpAscend;

impl SequenceHandler for JumpAscend {
    fn update(
        _character_input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let mut update = CharacterStatusUpdate::default();
        // TODO: Read Kinematics and switch to airborne when Y axis velocity is downwards.
        if character_status.object_status.sequence_state == SequenceState::End {
            update.object_status.sequence_id = Some(CharacterSequenceId::JumpAscend);
            update.object_status.sequence_state = Some(SequenceState::Begin);
        }

        update
    }
}

#[cfg(test)]
mod test {
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus,
            ObjectStatusUpdate, RunCounter,
        },
    };

    use super::JumpAscend;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(None, ObjectStatusUpdate::new(None, None, None)),
            JumpAscend::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(
                        CharacterSequenceId::JumpAscend,
                        SequenceState::Ongoing,
                        false
                    )
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn restarts_airborne_when_sequence_ends() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                None,
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::JumpAscend),
                    Some(SequenceState::Begin),
                    None
                )
            ),
            JumpAscend::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::JumpAscend, SequenceState::End, false)
                ),
                &Kinematics::default()
            )
        );
    }
}
