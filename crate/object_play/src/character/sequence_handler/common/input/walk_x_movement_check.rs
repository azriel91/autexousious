use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate, RunCounter},
};

use character::sequence_handler::{common::SequenceRepeat, SequenceHandler, SequenceHandlerUtil};

/// Determines whether to swithc to the `Walk` or `Run` sequence based on X input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub(crate) struct WalkXMovementCheck;

impl SequenceHandler for WalkXMovementCheck {
    fn update(
        input: &ControllerInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
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

            let sequence_id = match (character_status.run_counter, same_direction) {
                (RunCounter::Unused, _) | (RunCounter::Increase(_), false) => {
                    Some(CharacterSequenceId::Walk)
                }
                (RunCounter::Decrease(_), true) => Some(CharacterSequenceId::Run),
                (RunCounter::Exceeded, _)
                | (RunCounter::Decrease(_), false)
                | (RunCounter::Increase(_), true) => None,
            };

            let sequence_state = if sequence_id.is_some() {
                Some(SequenceState::Begin)
            } else {
                None
            };

            let grounding = None;

            if let (None, None) = (sequence_id, mirrored) {
                return SequenceRepeat::update(input, character_status, kinematics);
            }

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
            // The responsibility of switching to `Stand` is handled elsewhere.
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
            CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
            RunCounter,
        },
    };

    use super::WalkXMovementCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn none_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            WalkXMovementCheck::update(
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
    fn walk_non_mirror_when_x_axis_positive_mirror() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

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
            WalkXMovementCheck::update(
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
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    mirrored: Some(true),
                    ..Default::default()
                },
                ..Default::default()
            }),
            WalkXMovementCheck::update(
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
    fn restarts_walk_when_sequence_ended() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(CharacterStatusUpdate {
                        object_status: ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Walk),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    WalkXMovementCheck::update(
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
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Run),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            }),
            WalkXMovementCheck::update(
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
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Run),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            }),
            WalkXMovementCheck::update(
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
