use game_input::ControllerInput;
use object_model::entity::{CharacterStatus, Grounding, RunCounter};

use character::sequence_handler::SequenceHandlerUtil;

/// Determines how to update the `RunCounter`.
#[derive(Debug)]
pub(crate) struct RunCounterUpdater;

impl RunCounterUpdater {
    pub(crate) fn update(
        input: &ControllerInput,
        character_status: &CharacterStatus,
    ) -> Option<RunCounter> {
        if character_status.object_status.grounding != Grounding::OnGround
            || input.defend
            || input.jump
            || input.attack
        {
            if character_status.run_counter == RunCounter::Unused {
                return None;
            } else {
                return Some(RunCounter::Unused);
            }
        }

        use object_model::entity::RunCounter::*;
        if input.x_axis_value == 0. {
            match character_status.run_counter {
                Unused => None,
                Exceeded | Decrease(0) => Some(Unused),
                Decrease(ticks) => Some(Decrease(ticks - 1)),
                Increase(_) => Some(Decrease(RunCounter::RESET_TICK_COUNT)),
            }
        } else {
            let same_direction = SequenceHandlerUtil::input_matches_direction(
                input,
                character_status.object_status.mirrored,
            );
            match (character_status.run_counter, same_direction) {
                (Unused, _) | (Decrease(_), false) | (Increase(_), false) => {
                    Some(Increase(RunCounter::RESET_TICK_COUNT))
                }
                (Decrease(_), true) => Some(Unused), // Switch to running
                (Increase(0), true) => Some(Exceeded),
                (Increase(ticks), true) => Some(Increase(ticks - 1)),
                (Exceeded, _) => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::entity::{CharacterStatus, Grounding, ObjectStatus, RunCounter};

    use super::RunCounterUpdater;

    #[test]
    fn none_when_grounding_is_airborne_and_unused() {
        let input = ControllerInput::default();

        assert_eq!(
            None,
            RunCounterUpdater::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Unused,
                    object_status: ObjectStatus {
                        grounding: Grounding::Airborne,
                        ..Default::default()
                    }
                }
            )
        );
    }

    #[test]
    fn unused_when_grounding_is_airborne() {
        let input = ControllerInput::default();

        assert_eq!(
            Some(RunCounter::Unused),
            RunCounterUpdater::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(10),
                    object_status: ObjectStatus {
                        grounding: Grounding::Airborne,
                        ..Default::default()
                    }
                }
            )
        );
    }

    #[test]
    fn none_when_jump_is_pressed_and_unused() {
        let mut input = ControllerInput::default();
        input.jump = true;

        assert_eq!(
            None,
            RunCounterUpdater::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Unused,
                    object_status: ObjectStatus {
                        grounding: Grounding::Airborne,
                        ..Default::default()
                    }
                }
            )
        );
    }

    macro_rules! test_action_button {
        ($test_name:ident, $action_button:ident) => {
            #[test]
            fn $test_name() {
                let mut input = ControllerInput::default();
                input.$action_button = true;

                assert_eq!(
                    Some(RunCounter::Unused),
                    RunCounterUpdater::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Increase(10),
                            object_status: ObjectStatus {
                                grounding: Grounding::Airborne,
                                ..Default::default()
                            }
                        }
                    )
                );
            }
        };
    }

    test_action_button!(unused_when_defend_is_pressed, defend);
    test_action_button!(unused_when_jump_is_pressed, jump);
    test_action_button!(unused_when_attack_is_pressed, attack);

    #[test]
    fn none_when_unused_and_no_x_input() {
        let input = ControllerInput::default();

        assert_eq!(
            None,
            RunCounterUpdater::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Unused,
                    ..Default::default()
                }
            )
        );
    }

    #[test]
    fn unused_when_counter_decrease_runs_out_and_no_x_input() {
        let input = ControllerInput::default();

        assert_eq!(
            Some(RunCounter::Unused),
            RunCounterUpdater::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(0),
                    ..Default::default()
                }
            )
        );
    }

    #[test]
    fn decrements_run_counter_when_decrease_and_no_x_input() {
        let input = ControllerInput::default();

        assert_eq!(
            Some(RunCounter::Decrease(0)),
            RunCounterUpdater::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(1),
                    ..Default::default()
                }
            )
        );
    }

    #[test]
    fn decrease_when_increase_and_no_x_input() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
            RunCounterUpdater::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(0),
                    object_status: ObjectStatus {
                        ..Default::default()
                    }
                }
            )
        );
    }

    #[test]
    fn increase_when_unused_and_input_non_zero() {
        let x_inputs = vec![1., -1.];
        let mirrors = vec![false, true];

        x_inputs
            .into_iter()
            .zip(mirrors.into_iter())
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                    RunCounterUpdater::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Unused,
                            object_status: ObjectStatus {
                                mirrored,
                                ..Default::default()
                            }
                        }
                    )
                );
            });
    }

    #[test]
    fn increase_when_decrease_input_different_direction() {
        vec![(1., true), (-1., false)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                    RunCounterUpdater::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Decrease(11),
                            object_status: ObjectStatus {
                                mirrored,
                                ..Default::default()
                            }
                        }
                    )
                );
            });
    }

    #[test]
    fn increase_when_increase_input_different_direction() {
        vec![(1., true), (-1., false)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                    RunCounterUpdater::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Increase(11),
                            object_status: ObjectStatus {
                                mirrored,
                                ..Default::default()
                            }
                        }
                    )
                );
            });
    }

    #[test]
    fn unused_when_decrease_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(RunCounter::Unused),
                    RunCounterUpdater::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Decrease(11),
                            object_status: ObjectStatus {
                                mirrored,
                                ..Default::default()
                            }
                        }
                    )
                );
            });
    }

    #[test]
    fn exceeded_when_input_positive_same_direction_and_exceeds_tick_count() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(RunCounter::Exceeded),
                    RunCounterUpdater::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Increase(0),
                            object_status: ObjectStatus {
                                mirrored,
                                ..Default::default()
                            }
                        }
                    )
                );
            });
    }

    #[test]
    fn decrements_increase_value_when_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(RunCounter::Increase(10)),
                    RunCounterUpdater::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Increase(11),
                            object_status: ObjectStatus {
                                mirrored,
                                ..Default::default()
                            }
                        }
                    )
                );
            });
    }

    #[test]
    fn none_when_exceeded_and_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    None,
                    RunCounterUpdater::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Exceeded,
                            object_status: ObjectStatus {
                                mirrored,
                                ..Default::default()
                            }
                        }
                    )
                );
            });
    }
}
