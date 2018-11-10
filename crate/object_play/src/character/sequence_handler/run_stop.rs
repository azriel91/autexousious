use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, CharacterStatusUpdate, Kinematics},
};

use character::sequence_handler::{
    common::{grounding::AirborneCheck, status::AliveCheck},
    CharacterSequenceHandler, SequenceHandler,
};

#[derive(Debug)]
pub(crate) struct RunStop;

impl CharacterSequenceHandler for RunStop {
    fn update(
        input: &ControllerInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        [AliveCheck::update, AirborneCheck::update]
            .iter()
            .fold(None, |status_update, fn_update| {
                status_update.or_else(|| fn_update(input, character_status, kinematics))
            })
            .unwrap_or_else(|| {
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
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics, ObjectStatus,
            ObjectStatusUpdate,
        },
    };

    use super::RunStop;
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
            RunStop::update(
                &ControllerInput::default(),
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::RunStop,
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
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::default(),
            RunStop::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::RunStop,
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
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Stand),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            RunStop::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::RunStop,
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
