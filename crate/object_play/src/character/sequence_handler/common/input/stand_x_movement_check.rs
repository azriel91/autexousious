use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
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
                components.object_status.mirrored,
            );

            let mirrored = if same_direction {
                None
            } else {
                Some(!components.object_status.mirrored)
            };

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

            let sequence_state = Some(SequenceState::Begin);
            let grounding = None;

            Some(ObjectStatusUpdate::new(
                sequence_id,
                sequence_state,
                mirrored,
                grounding,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, Kinematics, Mirrored, ObjectStatus, ObjectStatusUpdate, RunCounter,
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
                &ObjectStatus::default(),
                &Kinematics::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn walk_non_mirror_when_x_axis_is_positive() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                mirrored: Some(Mirrored(false)),
                ..Default::default()
            }),
            StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    mirrored: Mirrored(true),
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            ))
        );

        // Already facing right
        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            }),
            StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    mirrored: Mirrored(false),
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn walk_mirror_when_x_axis_is_negative() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                mirrored: Some(Mirrored(true)),
                ..Default::default()
            }),
            StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    mirrored: Mirrored(false),
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            ))
        );

        // Already facing left
        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            }),
            StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    mirrored: Mirrored(true),
                    ..Default::default()
                },
                &Kinematics::default(),
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
                        sequence_state: Some(SequenceState::Begin),
                        ..Default::default()
                    }),
                    StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            mirrored: mirrored.into(),
                            ..Default::default()
                        },
                        &Kinematics::default(),
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
                        sequence_state: Some(SequenceState::Begin),
                        mirrored: Some(Mirrored(!mirrored)),
                        ..Default::default()
                    }),
                    StandXMovementCheck::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            mirrored: mirrored.into(),
                            ..Default::default()
                        },
                        &Kinematics::default(),
                        RunCounter::Decrease(10)
                    ))
                );
            });
    }
}
