use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate,
    },
};

use character::sequence_handler::CharacterSequenceHandler;

/// Hold forward to run, release to stop running.
#[derive(Debug)]
pub(crate) struct Run;

impl CharacterSequenceHandler for Run {
    fn update(
        input: &CharacterInput,
        character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        // Should always be `RunCounter::Unused`
        let run_counter = None;
        // Don't change facing direction
        let mirrored = None;

        let object_status = &character_status.object_status;
        let (sequence_id, sequence_state) = if (input.x_axis_value < 0. && object_status.mirrored)
            || (input.x_axis_value > 0. && !object_status.mirrored)
        {
            if character_status.object_status.sequence_state == SequenceState::End {
                (Some(CharacterSequenceId::Run), Some(SequenceState::Begin))
            } else {
                (None, None)
            }
        } else {
            (
                Some(CharacterSequenceId::StopRun),
                Some(SequenceState::Begin),
            )
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
            ObjectStatusUpdate,
        },
    };

    use super::Run;
    use character::sequence_handler::CharacterSequenceHandler;

    #[test]
    fn reverts_to_stop_run_when_no_input() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::StopRun),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Run::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Run,
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
    fn keeps_running_when_x_axis_positive_and_non_mirrored() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::default(),
            Run::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Run,
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
    fn keeps_running_when_x_axis_negative_and_mirrored() {
        let input = CharacterInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::default(),
            Run::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Run,
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
    fn restarts_run_when_sequence_ended() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = CharacterInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    CharacterStatusUpdate {
                        object_status: ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Run),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Run::update(
                        &input,
                        &CharacterStatus {
                            object_status: ObjectStatus {
                                sequence_id: CharacterSequenceId::Run,
                                sequence_state: SequenceState::End,
                                mirrored,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        &Kinematics::default()
                    )
                );
            });
    }

    #[test]
    fn reverts_to_stop_run_when_x_axis_negative_and_non_mirrored() {
        let input = CharacterInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::StopRun),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Run::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Run,
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
    fn reverts_to_stop_run_when_x_axis_positive_and_mirrored() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::StopRun),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Run::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Run,
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
    fn keeps_running_when_x_axis_positive_z_axis_non_zero_and_non_mirrored() {
        let input = CharacterInput::new(1., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::default(),
            Run::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Run,
                        mirrored: false,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );

        let input = CharacterInput::new(1., -1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::default(),
            Run::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Run,
                        mirrored: false,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }
}
