use game_input::ControllerInput;
use object_model::entity::{
    CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate,
};

use character::sequence_handler::{
    common::{
        grounding::AirborneCheck,
        input::{JumpCheck, StandAttackCheck, StandXMovementCheck, StandZMovementCheck},
        status::AliveCheck,
        util::RunCounterUpdater,
        SequenceRepeat,
    },
    CharacterSequenceHandler, SequenceHandler,
};

#[derive(Debug)]
pub(crate) struct Stand;

impl CharacterSequenceHandler for Stand {
    fn update(
        input: &ControllerInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        use object_model::entity::RunCounter::*;
        match character_status.run_counter {
            Exceeded | Increase(_) => panic!(
                "Invalid run_counter state during `Stand` sequence: `{:?}`",
                character_status.run_counter
            ),
            _ => {}
        };

        let run_counter = RunCounterUpdater::update(input, character_status);

        let status_update = [
            AliveCheck::update,
            AirborneCheck::update,
            JumpCheck::update,
            StandAttackCheck::update,
            StandXMovementCheck::update,
            StandZMovementCheck::update,
            SequenceRepeat::update,
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
            hp: None,
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
            CharacterStatus, CharacterStatusUpdate, Grounding, HealthPoints, Kinematics,
            ObjectStatus, ObjectStatusUpdate, RunCounter,
        },
    };

    use super::Stand;
    use character::sequence_handler::CharacterSequenceHandler;

    #[test]
    fn no_change_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::default(),
            Stand::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Unused,
                    hp: HealthPoints(100),
                    object_status: ObjectStatus::new(
                        CharacterSequenceId::Stand,
                        SequenceState::Ongoing,
                        true,
                        Grounding::OnGround
                    )
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn restarts_stand_when_no_input_and_sequence_end() {
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
            Stand::update(
                &input,
                &CharacterStatus {
                    hp: HealthPoints(100),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Stand,
                        sequence_state: SequenceState::End,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn switches_to_jump_descend_when_airborne() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus {
                    hp: HealthPoints(100),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Stand,
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
    fn switches_run_counter_to_unused_when_airborne() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Unused),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(10),
                    hp: HealthPoints(100),
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Stand,
                        grounding: Grounding::Airborne,
                        ..Default::default()
                    },
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn decrements_run_counter_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Decrease(0)),
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(1),
                    hp: HealthPoints(100),
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn switches_run_counter_to_unused_when_counter_runs_out() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Unused),
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(0),
                    hp: HealthPoints(100),
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    #[should_panic(expected = "Invalid run_counter state")]
    fn panics_when_run_counter_exceeded() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        Stand::update(
            &input,
            &CharacterStatus {
                run_counter: RunCounter::Exceeded,
                hp: HealthPoints(100),
                ..Default::default()
            },
            &Kinematics::default(),
        );
    } // kcov-ignore

    #[test]
    #[should_panic(expected = "Invalid run_counter state")]
    fn panics_when_run_counter_increase() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        Stand::update(
            &input,
            &CharacterStatus {
                run_counter: RunCounter::Increase(10),
                hp: HealthPoints(100),
                ..Default::default()
            },
            &Kinematics::default(),
        );
    } // kcov-ignore

    #[test]
    fn walk_non_mirror_when_x_axis_is_positive() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    mirrored: Some(false),
                    ..Default::default()
                },
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus {
                    hp: HealthPoints(100),
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
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus {
                    hp: HealthPoints(100),
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
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    mirrored: Some(true),
                    ..Default::default()
                },
                ..Default::default()
            },
            Stand::update(
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
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus {
                    hp: HealthPoints(100),
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
    fn walk_when_z_axis_is_non_zero_and_decrements_tick_count() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Decrease(9)),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(10),
                    hp: HealthPoints(100),
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_when_x_and_z_axes_are_non_zero() {
        let input = ControllerInput::new(1., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus {
                    hp: HealthPoints(100),
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
    fn run_when_run_counter_decrease_x_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    CharacterStatusUpdate {
                        run_counter: Some(RunCounter::Unused),
                        object_status: ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Run),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Stand::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Decrease(10),
                            hp: HealthPoints(100),
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
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    CharacterStatusUpdate {
                        run_counter: Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                        object_status: ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Walk),
                            sequence_state: Some(SequenceState::Begin),
                            mirrored: Some(!mirrored),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Stand::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Decrease(10),
                            hp: HealthPoints(100),
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
    fn jump_when_jump_is_pressed() {
        vec![(0., 0.), (1., 0.), (-1., 0.), (0., 1.)]
            .into_iter()
            .for_each(|(x_input, z_input)| {
                let input = ControllerInput::new(x_input, z_input, false, true, false, false);

                assert_eq!(
                    CharacterStatusUpdate {
                        object_status: ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Jump),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Stand::update(
                        &input,
                        &CharacterStatus {
                            hp: HealthPoints(100),
                            ..Default::default()
                        },
                        &Kinematics::default()
                    )
                );
            });
    }

    #[test]
    fn stand_attack_when_attack_is_pressed() {
        let mut input = ControllerInput::default();
        input.attack = true;

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::StandAttack),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus {
                    hp: HealthPoints(100),
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn switches_run_counter_to_unused_when_jump() {
        let input = ControllerInput::new(0., 0., false, true, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                run_counter: Some(RunCounter::Unused),
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Jump),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(0),
                    hp: HealthPoints(100),
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }
}
