use object_model::{
    config::object::CharacterSequenceId,
    entity::{ObjectStatusUpdate, RunCounter},
};

use character::sequence_handler::{SequenceHandler, SequenceHandlerUtil};
use CharacterSequenceUpdateComponents;

/// Determines whether to swithc to the `Walk` or `Run` sequence based on X input.
///
/// This should only be called from the Stand sequence handler.
#[derive(Debug)]
pub(crate) struct StandXMovementCheck;

impl SequenceHandler for StandXMovementCheck {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<ObjectStatusUpdate<CharacterSequenceId>> {
        if components.controller_input.x_axis_value != 0. {
            let same_direction = SequenceHandlerUtil::input_matches_direction(
                components.controller_input,
                components.mirrored,
            );

            let sequence_id = match components.run_counter {
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

            Some(ObjectStatusUpdate::new(sequence_id))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, ObjectStatusUpdate,
            RunCounter, SequenceStatus,
        },
    };

    use super::StandXMovementCheck;
    use character::sequence_handler::SequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_change_when_no_x_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn walk_when_x_axis_is_positive_mirrored() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
            }),
            StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored(true),
                Grounding::default(),
                RunCounter::default()
            ))
        );

        // Already facing right
        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
            }),
            StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn walk_when_x_axis_is_negative_non_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
            }),
            StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::default()
            ))
        );

        // Already facing left
        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
            }),
            StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Stand,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored(true),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn run_when_run_counter_decrease_x_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(ObjectStatusUpdate {
                        sequence_id: Some(CharacterSequenceId::Run),
                    }),
                    StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::Stand,
                        },
                        SequenceStatus::default(),
                        &Kinematics::default(),
                        mirrored.into(),
                        Grounding::default(),
                        RunCounter::Decrease(10)
                    ))
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
                    Some(ObjectStatusUpdate {
                        sequence_id: Some(CharacterSequenceId::Walk),
                    }),
                    StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::Stand,
                        },
                        SequenceStatus::default(),
                        &Kinematics::default(),
                        mirrored.into(),
                        Grounding::default(),
                        RunCounter::Decrease(10)
                    ))
                );
            });
    }
}
