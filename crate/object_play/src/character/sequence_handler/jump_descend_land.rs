use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics},
};

use character::sequence_handler::CharacterSequenceHandler;

#[derive(Debug)]
pub(crate) struct JumpDescendLand;

impl CharacterSequenceHandler for JumpDescendLand {
    fn update(
        _character_input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let mut update = CharacterStatusUpdate::default();
        if character_status.object_status.sequence_state == SequenceState::End {
            update.object_status.sequence_id = Some(CharacterSequenceId::Stand);
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
            ObjectStatusUpdate,
        },
    };

    use super::JumpDescendLand;
    use character::sequence_handler::CharacterSequenceHandler;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::default(),
            JumpDescendLand::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::JumpDescendLand,
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
            JumpDescendLand::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::JumpDescendLand,
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
