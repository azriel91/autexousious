use object_model::{config::object::CharacterSequenceId, entity::ObjectStatusUpdate};

use character::sequence_handler::{
    common::{
        grounding::AirborneCheck,
        input::{
            JumpCheck, StandAttackCheck, WalkNoMovementCheck, WalkXMovementCheck,
            WalkZMovementCheck,
        },
        status::AliveCheck,
    },
    CharacterSequenceHandler, SequenceHandler,
};
use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct Walk;

impl CharacterSequenceHandler for Walk {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        [
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
            status_update.or_else(|| fn_update(components))
        })
        .unwrap_or_else(|| ObjectStatusUpdate::default())
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, ObjectStatusUpdate,
            RunCounter, SequenceStatus,
        },
    };

    use super::Walk;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn reverts_to_stand_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Stand),
            },
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::Increase(10)
            ))
        );
    }

    #[test]
    fn reverts_to_stand_with_run_counter_unused_when_no_input_and_run_counter_exceeded() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Stand),
            },
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::Exceeded
            ))
        );
    }

    #[test]
    fn walk_when_x_axis_positive_mirror() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
            },
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored(true),
                Grounding::default(),
                RunCounter::Increase(11)
            ))
        );
    }

    #[test]
    fn walk_when_x_axis_negative_non_mirror() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
            },
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::Increase(11)
            ))
        );
    }

    #[test]
    fn walk_when_z_axis_non_zero() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate::default(),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::Increase(0)
            ))
        );

        let input = ControllerInput::new(0., -1., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate::default(),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::Increase(0)
            ))
        );
    }

    #[test]
    fn restarts_walk_when_sequence_ended() {
        vec![(0., 1.), (0., -1.)]
            .into_iter()
            .for_each(|(x_input, z_input)| {
                let input = ControllerInput::new(x_input, z_input, false, false, false, false);

                assert_eq!(
                    ObjectStatusUpdate {
                        sequence_id: Some(CharacterSequenceId::Walk),
                    },
                    Walk::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::Walk,
                        },
                        SequenceStatus::End,
                        &Kinematics::default(),
                        Mirrored(false),
                        Grounding::default(),
                        RunCounter::Increase(0)
                    ))
                );
            });

        vec![(1., 1., false), (-1., -1., true)]
            .into_iter()
            .for_each(|(x_input, z_input, mirrored)| {
                let input = ControllerInput::new(x_input, z_input, false, false, false, false);

                assert_eq!(
                    ObjectStatusUpdate {
                        sequence_id: Some(CharacterSequenceId::Walk),
                    },
                    Walk::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::Walk,
                        },
                        SequenceStatus::End,
                        &Kinematics::default(),
                        mirrored.into(),
                        Grounding::default(),
                        RunCounter::Increase(1)
                    ))
                );
            });
    }

    #[test]
    fn run_when_x_axis_positive_and_run_counter_decrease_non_mirror() {
        let input = ControllerInput::new(1., -1., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Run),
            },
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::Decrease(10)
            ))
        );
    }

    #[test]
    fn run_when_x_axis_negative_and_run_counter_decrease_mirror() {
        let input = ControllerInput::new(-1., -1., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Run),
            },
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored(true),
                Grounding::default(),
                RunCounter::Decrease(10)
            ))
        );
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
                    },
                    Walk::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus::default(),
                        SequenceStatus::default(),
                        &Kinematics::default(),
                        Mirrored::default(),
                        Grounding::default(),
                        RunCounter::default()
                    ))
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
            },
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus::default(),
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }
}
