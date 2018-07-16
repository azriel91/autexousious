use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics},
};

use character::sequence_handler::{SequenceHandler, SequenceHandlerUtil};

#[derive(Debug)]
pub(crate) struct JumpAscend;

impl SequenceHandler for JumpAscend {
    fn update(
        character_input: &CharacterInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let mut update = CharacterStatusUpdate::default();
        // Switch to jump_descend when Y axis velocity is no longer upwards.
        if kinematics.velocity[1] <= 0. {
            update.object_status.sequence_id = Some(CharacterSequenceId::JumpDescend);
            update.object_status.sequence_state = Some(SequenceState::Begin);
        } else if character_status.object_status.sequence_state == SequenceState::End {
            update.object_status.sequence_id = Some(CharacterSequenceId::JumpAscend);
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

    use super::JumpAscend;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = CharacterInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            CharacterStatusUpdate::default(),
            JumpAscend::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::JumpAscend,
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
    fn restarts_jump_ascend_when_sequence_ends() {
        let input = CharacterInput::new(0., 0., false, false, false, false);
        let mut kinematics = Kinematics::default();
        kinematics.velocity[1] = 1.;

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpAscend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            JumpAscend::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::JumpAscend,
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
    fn switches_to_jump_descend_when_y_velocity_is_zero_or_downwards() {
        let input = CharacterInput::new(0., 0., false, false, false, false);
        let mut downwards_kinematics = Kinematics::default();
        downwards_kinematics.velocity[1] = -1.;

        vec![Kinematics::default(), downwards_kinematics]
            .into_iter()
            .for_each(|kinematics| {
                assert_eq!(
                    CharacterStatusUpdate {
                        object_status: ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::JumpDescend),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    JumpAscend::update(
                        &input,
                        &CharacterStatus {
                            object_status: ObjectStatus {
                                sequence_id: CharacterSequenceId::JumpAscend,
                                sequence_state: SequenceState::Ongoing,
                                grounding: Grounding::Airborne,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        &kinematics
                    )
                );
            });
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
                    JumpAscend::update(
                        &input,
                        &CharacterStatus {
                            object_status: ObjectStatus {
                                sequence_id: CharacterSequenceId::JumpAscend,
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
