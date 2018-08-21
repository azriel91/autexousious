use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate,
        RunCounter,
    },
};

use character::sequence_handler::{SequenceHandler, SequenceHandlerUtil};

/// Determines whether to swithc to the `Walk` or `Run` sequence based on X input.
///
/// This should only be called from the Stand or Walk sequence handlers.
#[derive(Debug)]
pub(crate) struct StandXMovementCheck;

impl SequenceHandler for StandXMovementCheck {
    fn update(
        input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> Option<CharacterStatusUpdate> {
        if input.x_axis_value != 0. {
            let same_direction = SequenceHandlerUtil::input_matches_direction(
                input,
                character_status.object_status.mirrored,
            );

            let mirrored = if same_direction {
                None
            } else {
                Some(!character_status.object_status.mirrored)
            };

            let sequence_id = match character_status.run_counter {
                RunCounter::Unused => Some(CharacterSequenceId::Walk),
                RunCounter::Decrease(_) => {
                    if same_direction {
                        Some(CharacterSequenceId::Run)
                    } else {
                        Some(CharacterSequenceId::Walk)
                    }
                }
                _ => unreachable!(), // kcov-ignore
            };

            let sequence_state = Some(SequenceState::Begin);
            let grounding = None;

            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate::new(
                    sequence_id,
                    sequence_state,
                    mirrored,
                    grounding,
                ),
                ..Default::default()
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus,
            ObjectStatusUpdate, RunCounter,
        },
    };

    use super::StandXMovementCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_change_when_no_x_input() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            StandXMovementCheck::update(
                &input,
                &CharacterStatus::default(),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_non_mirror_when_x_axis_is_positive() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    mirrored: Some(false),
                    ..Default::default()
                },
                ..Default::default()
            }),
            StandXMovementCheck::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        mirrored: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );

        // Already facing right
        assert_eq!(
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            }),
            StandXMovementCheck::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        mirrored: false,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_mirror_when_x_axis_is_negative() {
        let input = CharacterInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    mirrored: Some(true),
                    ..Default::default()
                },
                ..Default::default()
            }),
            StandXMovementCheck::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        mirrored: false,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );

        // Already facing left
        assert_eq!(
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            }),
            StandXMovementCheck::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        mirrored: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_when_run_counter_decrease_x_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = CharacterInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(CharacterStatusUpdate {
                        object_status: ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Run),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    StandXMovementCheck::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Decrease(10),
                            object_status: ObjectStatus {
                                mirrored,
                                ..Default::default()
                            },
                        },
                        &Kinematics::default()
                    )
                );
            });
    }

    #[test]
    fn walk_when_run_counter_decrease_x_input_different_direction() {
        vec![(1., true), (-1., false)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = CharacterInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(CharacterStatusUpdate {
                        object_status: ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Walk),
                            sequence_state: Some(SequenceState::Begin),
                            mirrored: Some(!mirrored),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    StandXMovementCheck::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Decrease(10),
                            object_status: ObjectStatus {
                                mirrored,
                                ..Default::default()
                            },
                        },
                        &Kinematics::default()
                    )
                );
            });
    }
}
