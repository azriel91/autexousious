use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate,
        RunCounter,
    },
};

use character::sequence_handler::SequenceHandler;

#[derive(Debug)]
pub(crate) struct Stand;

impl SequenceHandler for Stand {
    fn update(
        input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        let (run_counter, mut sequence_id, mirrored) = {
            let mirrored = character_status.object_status.mirrored;

            use object_model::entity::RunCounter::*;
            match character_status.run_counter {
                Exceeded | Increase(_) => panic!(
                    "Invalid run_counter state during `Stand` sequence: `{:?}`",
                    character_status.run_counter
                ),
                _ => {}
            };

            // TODO: Don't handle action buttons in `SequenceHandler`s. Instead, each sequence has
            // default sequence update IDs for each action button, which are overridden by
            // configuration.
            if input.jump {
                let run_counter = if character_status.run_counter == Unused {
                    None
                } else {
                    Some(Unused)
                };
                return CharacterStatusUpdate::new(
                    run_counter,
                    ObjectStatusUpdate::new(
                        Some(CharacterSequenceId::Jump),
                        Some(SequenceState::Begin),
                        None,
                    ),
                );
            }

            if input.x_axis_value == 0. {
                let run_counter = match character_status.run_counter {
                    Unused => None,
                    Decrease(0) => Some(Unused),
                    Decrease(ticks) => Some(Decrease(ticks - 1)),
                    _ => unreachable!(),
                };
                let sequence_id =
                    if character_status.object_status.sequence_state == SequenceState::End {
                        Some(CharacterSequenceId::Stand)
                    } else {
                        None
                    };
                (run_counter, sequence_id, None)
            } else {
                let same_direction =
                    input.x_axis_value > 0. && !mirrored || input.x_axis_value < 0. && mirrored;

                match (character_status.run_counter, same_direction) {
                    (Unused, false) | (Decrease(_), false) => (
                        Some(Increase(RunCounter::RESET_TICK_COUNT)),
                        Some(CharacterSequenceId::Walk),
                        Some(!mirrored),
                    ),
                    (Unused, true) => (
                        Some(Increase(RunCounter::RESET_TICK_COUNT)),
                        Some(CharacterSequenceId::Walk),
                        None,
                    ),
                    (Decrease(_), true) => (Some(Unused), Some(CharacterSequenceId::Run), None),
                    _ => unreachable!(),
                }
            }
        };

        // If we aren't already running, then we can walk
        if sequence_id.is_none() && input.z_axis_value != 0. {
            sequence_id = Some(CharacterSequenceId::Walk);
        }

        let sequence_state = if sequence_id.is_some() {
            Some(SequenceState::Begin)
        } else {
            None
        };

        CharacterStatusUpdate::new(
            run_counter,
            ObjectStatusUpdate::new(sequence_id, sequence_state, mirrored),
        )
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

    use super::Stand;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn no_change_when_no_input() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(None, ObjectStatusUpdate::new(None, None, None)),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, true)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn restarts_stand_when_no_input_and_sequence_end() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                None,
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Stand),
                    Some(SequenceState::Begin),
                    None
                )
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::End, true)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn decrements_run_counter_when_no_input() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Decrease(0)),
                ObjectStatusUpdate::new(None, None, None)
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Decrease(1),
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, true)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn switches_run_counter_to_unused_when_no_counter_runs_out() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Unused),
                ObjectStatusUpdate::new(None, None, None)
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Decrease(0),
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, true)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_non_mirror_when_x_axis_is_positive() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Walk),
                    Some(SequenceState::Begin),
                    Some(false)
                )
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, true)
                ),
                &Kinematics::default()
            )
        );

        // Already facing right
        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Walk),
                    Some(SequenceState::Begin),
                    None
                )
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, false)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_mirror_when_x_axis_is_negative() {
        let input = CharacterInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Walk),
                    Some(SequenceState::Begin),
                    Some(true)
                )
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, false)
                ),
                &Kinematics::default()
            )
        );

        // Already facing left
        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Walk),
                    Some(SequenceState::Begin),
                    None
                )
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, true)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_when_z_axis_is_non_zero_and_decrements_tick_count() {
        let input = CharacterInput::new(0., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Decrease(9)),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Walk),
                    Some(SequenceState::Begin),
                    None
                )
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Decrease(10),
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, true)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_when_x_and_z_axes_are_non_zero() {
        let input = CharacterInput::new(1., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Walk),
                    Some(SequenceState::Begin),
                    None
                )
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, false)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_when_x_axis_is_positive_and_run_counter_decrease_non_mirror() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Unused),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Run),
                    Some(SequenceState::Begin),
                    None
                )
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Decrease(10),
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, false)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_when_x_axis_is_positive_and_run_counter_decrease_mirror() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Walk),
                    Some(SequenceState::Begin),
                    Some(false)
                )
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Decrease(10),
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, true)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_when_x_axis_is_negative_and_run_counter_decrease_mirror() {
        let input = CharacterInput::new(-1., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Unused),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Run),
                    Some(SequenceState::Begin),
                    None
                )
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Decrease(10),
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, true)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn walk_when_x_axis_is_negative_and_run_counter_decrease_non_mirror() {
        let input = CharacterInput::new(-1., -1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                Some(RunCounter::Increase(RunCounter::RESET_TICK_COUNT)),
                ObjectStatusUpdate::new(
                    Some(CharacterSequenceId::Walk),
                    Some(SequenceState::Begin),
                    Some(true)
                )
            ),
            Stand::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Decrease(10),
                    ObjectStatus::new(CharacterSequenceId::Stand, SequenceState::Ongoing, false)
                ),
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn jump_when_jump_is_pressed() {
        vec![(0., 0.), (1., 0.), (-1., 0.), (0., 1.)]
            .into_iter()
            .for_each(|(x_input, z_input)| {
                let input = CharacterInput::new(x_input, z_input, false, true, false, false);

                assert_eq!(
                    CharacterStatusUpdate::new(
                        Some(RunCounter::Unused),
                        ObjectStatusUpdate::new(
                            Some(CharacterSequenceId::Jump),
                            Some(SequenceState::Begin),
                            None
                        )
                    ),
                    Stand::update(
                        &input,
                        &CharacterStatus::new(
                            RunCounter::Decrease(1),
                            ObjectStatus::new(
                                CharacterSequenceId::Stand,
                                SequenceState::Ongoing,
                                false
                            )
                        ),
                        &Kinematics::default()
                    )
                );
            });
    }
}
