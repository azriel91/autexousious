use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate,
        RunCounter,
    },
};

use character::sequence_handler::{SequenceHandler, SequenceHandlerUtil};

#[derive(Debug)]
pub(crate) struct Walk;

impl SequenceHandler for Walk {
    fn update(
        input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let (run_counter, mut sequence_id, mirrored) = {
            let mirrored = character_status.object_status.mirrored;

            use object_model::entity::RunCounter::*;
            if input.x_axis_value == 0. {
                let run_counter = match character_status.run_counter {
                    Unused => None,
                    Exceeded | Decrease(0) => Some(Unused),
                    Decrease(ticks) => Some(Decrease(ticks - 1)),
                    Increase(_) => Some(Decrease(RunCounter::RESET_TICK_COUNT)),
                };
                (run_counter, Some(CharacterSequenceId::Stand), None)
            } else {
                let same_direction = SequenceHandlerUtil::input_matches_direction(input, mirrored);
                match (character_status.run_counter, same_direction) {
                    (Unused, _) | (Decrease(_), false) | (Increase(_), false) => (
                        Some(Increase(RunCounter::RESET_TICK_COUNT)),
                        Some(CharacterSequenceId::Walk),
                        Some(!mirrored),
                    ),
                    (Decrease(_), true) => (Some(Unused), Some(CharacterSequenceId::Run), None),
                    (Increase(0), true) => (Some(Exceeded), None, None),
                    (Increase(ticks), true) => (Some(Increase(ticks - 1)), None, None),
                    (Exceeded, _) => (None, None, None),
                }
            }
        };

        // If we are about to stand, but have z axis input, then we walk instead
        if sequence_id == Some(CharacterSequenceId::Stand) && input.z_axis_value != 0. {
            sequence_id = None;
        }

        // If we're maintaining the `Walk` state, and have reached the end of the sequence, restart.
        if sequence_id.is_none()
            && character_status.object_status.sequence_state == SequenceState::End
        {
            sequence_id = Some(CharacterSequenceId::Walk);
        }

        let sequence_state = if sequence_id.is_some() {
            Some(SequenceState::Begin)
        } else {
            None
        };

        // TODO: switch to `JumpDescend` when `Airborne`.
        let grounding = None;

        CharacterStatusUpdate {
            run_counter,
            object_status: ObjectStatusUpdate {
                sequence_id,
                sequence_state,
                mirrored,
                grounding,
            },
        }
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

    use super::Walk;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn reverts_to_stand_when_no_input() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

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
        let input = CharacterInput::new(0., 0., false, false, false, false);

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
        let input = CharacterInput::new(1., 0., false, false, false, false);

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
        let input = CharacterInput::new(1., 0., false, false, false, false);

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
        let input = CharacterInput::new(-1., 0., false, false, false, false);

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
        let input = CharacterInput::new(-1., 0., false, false, false, false);

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
        let input = CharacterInput::new(0., 1., false, false, false, false);

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
        let input = CharacterInput::new(0., 1., false, false, false, false);

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
        let input = CharacterInput::new(1., 1., false, false, false, false);

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
        let input = CharacterInput::new(1., 0., false, false, false, false);

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
        let input = CharacterInput::new(-1., 0., false, false, false, false);

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
        let input = CharacterInput::new(0., 1., false, false, false, false);

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

        let input = CharacterInput::new(0., -1., false, false, false, false);

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
                let input = CharacterInput::new(x_input, z_input, false, false, false, false);

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
                let input = CharacterInput::new(x_input, z_input, false, false, false, false);

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
        let input = CharacterInput::new(1., -1., false, false, false, false);

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
        let input = CharacterInput::new(-1., -1., false, false, false, false);

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
