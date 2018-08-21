use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics},
};

use character::sequence_handler::{
    common::grounding::AirborneCheck, CharacterSequenceHandler, SequenceHandler,
};

#[derive(Debug)]
pub(crate) struct StopRun;

impl CharacterSequenceHandler for StopRun {
    fn update(
        character_input: &CharacterInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        AirborneCheck::update(character_input, character_status, kinematics).unwrap_or_else(|| {
            let mut update = CharacterStatusUpdate::default();
            if character_status.object_status.sequence_state == SequenceState::End {
                update.object_status.sequence_id = Some(CharacterSequenceId::Stand);
                update.object_status.sequence_state = Some(SequenceState::Begin);
            }
            update
        })
    }
}

#[cfg(test)]
mod test {
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterInput, CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics,
            ObjectStatus, ObjectStatusUpdate,
        },
    };

    use super::StopRun;
    use character::sequence_handler::CharacterSequenceHandler;

    #[test]
    fn jump_descend_when_airborne() {
        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            StopRun::update(
                &CharacterInput::default(),
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::StopRun,
                        grounding: Grounding::Airborne,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::default(),
            StopRun::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::StopRun,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn reverts_to_stand_when_sequence_ended() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Stand),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            StopRun::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::StopRun,
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
