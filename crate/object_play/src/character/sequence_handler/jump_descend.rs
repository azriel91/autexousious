use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics},
};

use character::sequence_handler::{CharacterSequenceHandler, SequenceHandlerUtil};

#[derive(Debug)]
pub(crate) struct JumpDescend;

impl CharacterSequenceHandler for JumpDescend {
    fn update(
        character_input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let mut update = CharacterStatusUpdate::default();
        if character_status.object_status.grounding == Grounding::OnGround {
            update.object_status.sequence_id = Some(CharacterSequenceId::JumpDescendLand);
            update.object_status.sequence_state = Some(SequenceState::Begin);
        } else if character_status.object_status.sequence_state == SequenceState::End {
            update.object_status.sequence_id = Some(CharacterSequenceId::JumpDescend);
            update.object_status.sequence_state = Some(SequenceState::Begin);
        }

        // Switch direction if user is pressing the opposite way.
        if SequenceHandlerUtil::input_opposes_direction(
            character_input,
            character_status.object_status.mirrored,
        ) {
            update.object_status.mirrored = Some(!character_status.object_status.mirrored);
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
    use character::sequence_handler::CharacterSequenceHandler;

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

    #[test]
    fn jump_descend_land_when_on_ground() {
        let input = CharacterInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = -1.;

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpDescendLand),
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
                        grounding: Grounding::OnGround,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &kinematics
            )
        );
    }

    #[test]
    fn switches_mirror_when_pressing_opposite_direction() {
        vec![(-1., false), (1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = CharacterInput::new(x_input, 0., false, false, false, false);
                let mut kinematics = Kinematics::default();
                kinematics.velocity[1] = 1.;

                assert_eq!(
                    CharacterStatusUpdate {
                        object_status: ObjectStatusUpdate {
                            mirrored: Some(!mirrored),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    JumpDescend::update(
                        &input,
                        &CharacterStatus {
                            object_status: ObjectStatus {
                                sequence_id: CharacterSequenceId::JumpDescend,
                                grounding: Grounding::Airborne,
                                mirrored,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        &kinematics
                    )
                );
            });
    }
}
