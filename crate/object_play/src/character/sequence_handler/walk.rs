use game_input::ControllerInput;
use object_model::entity::{
    CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate,
};

use character::sequence_handler::{
    common::{
        grounding::AirborneCheck,
        input::{JumpCheck, WalkNoMovementCheck, WalkXMovementCheck, WalkZMovementCheck},
        util::RunCounterUpdater,
    },
    CharacterSequenceHandler, SequenceHandler,
};

#[derive(Debug)]
pub(crate) struct Walk;

impl CharacterSequenceHandler for Walk {
    fn update(
        input: &ControllerInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let run_counter = RunCounterUpdater::update(input, character_status);

        let status_update = [
            AirborneCheck::update,
            JumpCheck::update,
            WalkNoMovementCheck::update,
            WalkXMovementCheck::update,
            WalkZMovementCheck::update,
        ]
            .iter()
            .fold(None, |status_update, fn_update| {
                status_update.or_else(|| fn_update(input, character_status, kinematics))
            });

        if let Some(mut status_update) = status_update {
            status_update.run_counter = run_counter;
            return status_update;
        }

        CharacterStatusUpdate {
            run_counter,
            object_status: ObjectStatusUpdate::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
            RunCounter,
        },
    };

    use super::Walk;
    use character::sequence_handler::CharacterSequenceHandler;

    #[test]
    fn reverts_to_stand_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Stand),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(10),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn reverts_to_stand_with_run_counter_unused_when_no_input_and_run_counter_exceeded() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Unused),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Stand),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Exceeded,
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn decrements_increase_run_counter_when_x_axis_positive_non_mirror() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Increase(10)),
                ..Default::default()
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(11),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        mirrored: false,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_counter_exceeded_when_x_axis_positive_non_mirror_and_exceeds_tick_count() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Exceeded),
                ..Default::default()
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(0),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        mirrored: false,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn decrements_increase_run_counter_when_x_axis_negative_mirror() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Increase(10)),
                ..Default::default()
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(11),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        mirrored: true,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_counter_exceeded_when_x_axis_negative_mirror_and_exceeds_tick_count() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Exceeded),
                ..Default::default()
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(0),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        mirrored: true,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_counter_decrease_when_x_axis_zero_z_axis_positive_and_run_counter_increase() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                ..Default::default()
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(0),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn decrements_decrease_run_counter_when_z_axis_non_zero() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Decrease(10)),
                ..Default::default()
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(11),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn no_change_to_run_counter_when_exceeded() {
        let input = ControllerInput::new(1., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::default(),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Exceeded,
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        mirrored: false,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_non_mirror_when_x_axis_positive_mirror() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    mirrored: Some(false),
                    ..Default::default()
                }
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(11),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        mirrored: true,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_mirror_when_x_axis_negative_non_mirror() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    mirrored: Some(true),
                    ..Default::default()
                }
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(11),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        mirrored: false,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_when_z_axis_non_zero() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                ..Default::default()
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(0),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );

        let input = ControllerInput::new(0., -1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                ..Default::default()
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(0),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn restarts_walk_when_sequence_ended() {
        vec![(0., 1.), (0., -1.)]
            .into_iter()
            .for_each(|(x_input, z_input)| {
                let input = ControllerInput::new(x_input, z_input, false, false, false, false);

                assert_eq!(
                    CharacterStatusUpdate {
                        run_counter: Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                        object_status: ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Walk),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        }
                    },
                    Walk::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Increase(0),
                            object_status: ObjectStatus {
                                sequence_id: CharacterSequenceId::Walk,
                                sequence_state: SequenceState::End,
                                mirrored: false,
                                ..Default::default()
                            }
                        },
                        &Kinematics::default()
                    )
                );
            });

        vec![(1., 1., false), (-1., -1., true)]
            .into_iter()
            .for_each(|(x_input, z_input, mirrored)| {
                let input = ControllerInput::new(x_input, z_input, false, false, false, false);

                assert_eq!(
                    CharacterStatusUpdate {
                        run_counter: Some(RunCounter::Increase(0)),
                        object_status: ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Walk),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        }
                    },
                    Walk::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Increase(1),
                            object_status: ObjectStatus {
                                sequence_id: CharacterSequenceId::Walk,
                                sequence_state: SequenceState::End,
                                mirrored,
                                ..Default::default()
                            }
                        },
                        &Kinematics::default()
                    )
                );
            });
    }

    #[test]
    fn run_when_x_axis_positive_and_run_counter_decrease_non_mirror() {
        let input = ControllerInput::new(1., -1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Unused),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Run),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(10),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        mirrored: false,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_when_x_axis_negative_and_run_counter_decrease_mirror() {
        let input = ControllerInput::new(-1., -1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Unused),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Run),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            },
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(10),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        mirrored: true,
                        ..Default::default()
                    }
                },
                &Kinematics::default()
            )
        );
    }
}
