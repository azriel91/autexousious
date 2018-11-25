use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterStatus, Kinematics, ObjectStatus, ObjectStatusUpdate, RunCounter},
};

use character::sequence_handler::{
    common::{
        grounding::AirborneCheck,
        input::{JumpCheck, StandAttackCheck, StandXMovementCheck, StandZMovementCheck},
        status::AliveCheck,
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
        object_status: &ObjectStatus<CharacterSequenceId>,
        kinematics: &Kinematics<f32>,
        run_counter: RunCounter,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        use object_model::entity::RunCounter::*;
        match run_counter {
            Exceeded | Increase(_) => panic!(
                "Invalid run_counter state during `Stand` sequence: `{:?}`",
                run_counter
            ),
            _ => {}
        };

        [
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
            status_update.or_else(|| {
                fn_update(
                    input,
                    character_status,
                    object_status,
                    kinematics,
                    run_counter,
                )
            })
        })
        .unwrap_or_else(|| ObjectStatusUpdate::default())
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, ObjectStatusUpdate,
            RunCounter,
        },
    };

    use super::Stand;
    use character::sequence_handler::CharacterSequenceHandler;

    #[test]
    fn no_change_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate::default(),
            Stand::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus::new(
                    CharacterSequenceId::Stand,
                    SequenceState::Ongoing,
                    Mirrored(true),
                    Grounding::OnGround
                ),
                &Kinematics::default(),
                RunCounter::Unused
            )
        );
    }

    #[test]
    fn restarts_stand_when_no_input_and_sequence_end() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Stand),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                    sequence_state: SequenceState::End,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn switches_to_jump_descend_when_airborne() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::JumpDescend),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                    grounding: Grounding::Airborne,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    #[should_panic(expected = "Invalid run_counter state")]
    fn panics_when_run_counter_exceeded() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        Stand::update(
            &input,
            &CharacterStatus::default(),
            &ObjectStatus::default(),
            &Kinematics::default(),
            RunCounter::Exceeded,
        );
    } // kcov-ignore

    #[test]
    #[should_panic(expected = "Invalid run_counter state")]
    fn panics_when_run_counter_increase() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        Stand::update(
            &input,
            &CharacterStatus::default(),
            &ObjectStatus::default(),
            &Kinematics::default(),
            RunCounter::Increase(10),
        );
    } // kcov-ignore

    #[test]
    fn walk_non_mirror_when_x_axis_is_positive() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                mirrored: Some(Mirrored(false)),
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    mirrored: Mirrored(true),
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );

        // Already facing right
        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    mirrored: Mirrored(false),
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn walk_mirror_when_x_axis_is_negative() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                mirrored: Some(Mirrored(true)),
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    mirrored: Mirrored(false),
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );

        // Already facing left
        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    mirrored: Mirrored(true),
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn walk_when_x_and_z_axes_are_non_zero() {
        let input = ControllerInput::new(1., 1., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    mirrored: Mirrored(false),
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
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
                    ObjectStatusUpdate {
                        sequence_id: Some(CharacterSequenceId::Run),
                        sequence_state: Some(SequenceState::Begin),
                        ..Default::default()
                    },
                    Stand::update(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            mirrored: mirrored.into(),
                            ..Default::default()
                        },
                        &Kinematics::default(),
                        RunCounter::Decrease(10)
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
                    ObjectStatusUpdate {
                        sequence_id: Some(CharacterSequenceId::Walk),
                        sequence_state: Some(SequenceState::Begin),
                        mirrored: Some(Mirrored(!mirrored)),
                        ..Default::default()
                    },
                    Stand::update(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            mirrored: mirrored.into(),
                            ..Default::default()
                        },
                        &Kinematics::default(),
                        RunCounter::Decrease(10)
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
                    ObjectStatusUpdate {
                        sequence_id: Some(CharacterSequenceId::Jump),
                        sequence_state: Some(SequenceState::Begin),
                        ..Default::default()
                    },
                    Stand::update(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus::default(),
                        &Kinematics::default(),
                        RunCounter::default()
                    )
                );
            });
    }

    #[test]
    fn stand_attack_when_attack_is_pressed() {
        let mut input = ControllerInput::default();
        input.attack = true;

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::StandAttack),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            },
            Stand::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus::default(),
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }
}
