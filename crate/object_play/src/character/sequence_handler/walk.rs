use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{
        CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
    },
};

use character::sequence_handler::{
    common::{
        grounding::AirborneCheck,
        input::{
            JumpCheck, StandAttackCheck, WalkNoMovementCheck, WalkXMovementCheck,
            WalkZMovementCheck,
        },
        status::AliveCheck,
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
        object_status: &ObjectStatus<CharacterSequenceId>,
        kinematics: &Kinematics<f32>,
    ) -> (
        CharacterStatusUpdate,
        ObjectStatusUpdate<CharacterSequenceId>,
    ) {
        let run_counter = RunCounterUpdater::update(input, character_status, object_status);

        let status_update = [
            AliveCheck::update,
            AirborneCheck::update,
            JumpCheck::update,
            StandAttackCheck::update,
            WalkNoMovementCheck::update,
            WalkXMovementCheck::update,
            WalkZMovementCheck::update,
        ]
        .iter()
        .fold(None, |status_update, fn_update| {
            status_update.or_else(|| fn_update(input, character_status, object_status, kinematics))
        });

        if let Some((mut character_status_update, object_status_update)) = status_update {
            character_status_update.run_counter = run_counter;
            return (character_status_update, object_status_update);
        }

        (
            CharacterStatusUpdate {
                run_counter,
                hp: None,
            },
            ObjectStatusUpdate::default(),
        )
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, CharacterStatusUpdate, HealthPoints, Kinematics, ObjectStatus,
            ObjectStatusUpdate, RunCounter,
        },
    };

    use super::Walk;
    use character::sequence_handler::CharacterSequenceHandler;

    #[test]
    fn reverts_to_stand_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                    ..Default::default()
                },
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Stand),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(10),
                    hp: HealthPoints(100)
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn reverts_to_stand_with_run_counter_unused_when_no_input_and_run_counter_exceeded() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Unused),
                    ..Default::default()
                },
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Stand),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Exceeded,
                    hp: HealthPoints(100)
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn decrements_increase_run_counter_when_x_axis_positive_non_mirror() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Increase(10)),
                    ..Default::default()
                },
                ObjectStatusUpdate::default()
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(11),
                    hp: HealthPoints(100),
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    mirrored: false,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_counter_exceeded_when_x_axis_positive_non_mirror_and_exceeds_tick_count() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Exceeded),
                    ..Default::default()
                },
                ObjectStatusUpdate::default()
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(0),
                    hp: HealthPoints(100),
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    mirrored: false,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn decrements_increase_run_counter_when_x_axis_negative_mirror() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Increase(10)),
                    ..Default::default()
                },
                ObjectStatusUpdate::default()
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(11),
                    hp: HealthPoints(100),
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    mirrored: true,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_counter_exceeded_when_x_axis_negative_mirror_and_exceeds_tick_count() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Exceeded),
                    ..Default::default()
                },
                ObjectStatusUpdate::default()
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(0),
                    hp: HealthPoints(100),
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    mirrored: true,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_counter_decrease_when_x_axis_zero_z_axis_positive_and_run_counter_increase() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                    ..Default::default()
                },
                ObjectStatusUpdate::default()
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(0),
                    hp: HealthPoints(100)
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn decrements_decrease_run_counter_when_z_axis_non_zero() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Decrease(10)),
                    ..Default::default()
                },
                ObjectStatusUpdate::default()
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(11),
                    hp: HealthPoints(100)
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn no_change_to_run_counter_when_exceeded() {
        let input = ControllerInput::new(1., 1., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::default()
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Exceeded,
                    hp: HealthPoints(100),
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    mirrored: false,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_non_mirror_when_x_axis_positive_mirror() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                    ..Default::default()
                },
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    mirrored: Some(false),
                    ..Default::default()
                }
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(11),
                    hp: HealthPoints(100),
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    mirrored: true,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_mirror_when_x_axis_negative_non_mirror() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                    ..Default::default()
                },
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    mirrored: Some(true),
                    ..Default::default()
                }
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(11),
                    hp: HealthPoints(100),
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    mirrored: false,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_when_z_axis_non_zero() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                    ..Default::default()
                },
                ObjectStatusUpdate::default()
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(0),
                    hp: HealthPoints(100)
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );

        let input = ControllerInput::new(0., -1., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                    ..Default::default()
                },
                ObjectStatusUpdate::default()
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Increase(0),
                    hp: HealthPoints(100)
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    ..Default::default()
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
                    (
                        CharacterStatusUpdate {
                            run_counter: Some(RunCounter::Decrease(RunCounter::RESET_TICK_COUNT)),
                            ..Default::default()
                        },
                        ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Walk),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        }
                    ),
                    Walk::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Increase(0),
                            ..Default::default()
                        },
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::Walk,
                            sequence_state: SequenceState::End,
                            mirrored: false,
                            ..Default::default()
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
                    (
                        CharacterStatusUpdate {
                            run_counter: Some(RunCounter::Increase(0)),
                            ..Default::default()
                        },
                        ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Walk),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        }
                    ),
                    Walk::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Increase(1),
                            hp: HealthPoints(100)
                        },
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::Walk,
                            sequence_state: SequenceState::End,
                            mirrored,
                            ..Default::default()
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
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Unused),
                    ..Default::default()
                },
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Run),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(10),
                    hp: HealthPoints(100),
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    mirrored: false,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_when_x_axis_negative_and_run_counter_decrease_mirror() {
        let input = ControllerInput::new(-1., -1., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate {
                    run_counter: Some(RunCounter::Unused),
                    ..Default::default()
                },
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Run),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            Walk::update(
                &input,
                &CharacterStatus {
                    run_counter: RunCounter::Decrease(10),
                    hp: HealthPoints(100),
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    mirrored: true,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn jump_when_jump_is_pressed() {
        vec![(0., 0.), (1., 0.), (-1., 0.), (0., 1.)]
            .into_iter()
            .for_each(|(x_input, z_input)| {
                let input = ControllerInput::new(x_input, z_input, false, true, false, false);

                assert_eq!(
                    (
                        CharacterStatusUpdate::default(),
                        ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Jump),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        }
                    ),
                    Walk::update(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus::default(),
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
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::StandAttack),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            Walk::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus::default(),
                &Kinematics::default()
            )
        );
    }
}
