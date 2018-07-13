use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics},
};

use character::sequence_handler::SequenceHandler;

#[derive(Debug)]
pub(crate) struct JumpDescend;

impl SequenceHandler for JumpDescend {
    fn update(
        _character_input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let mut update = CharacterStatusUpdate::default();
        // TODO: Check if character is on ground, and if so, switch to `JumpDescendLand`.
        if character_status.object_status.sequence_state == SequenceState::End {
            update.object_status.sequence_id = Some(CharacterSequenceId::JumpDescend);
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
            CharacterInput, CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics,
            ObjectStatus, ObjectStatusUpdate,
        },
    };

    use super::JumpDescend;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = CharacterInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            CharacterStatusUpdate::default(),
            JumpDescend::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::JumpDescend,
                        grounding: Grounding::Airborne,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &kinematics
            )
        );
    }

    #[test]
    fn restarts_jump_descend_when_sequence_ends() {
        let input = CharacterInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            JumpDescend::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::JumpDescend,
                        sequence_state: SequenceState::End,
                        grounding: Grounding::Airborne,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &kinematics
            )
        );
    }
}
