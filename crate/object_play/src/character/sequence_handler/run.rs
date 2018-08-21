use object_model::entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate, Kinematics};

use character::sequence_handler::{
    common::{grounding::AirborneCheck, input::RunStopCheck},
    CharacterSequenceHandler, SequenceHandler,
};

/// Hold forward to run, release to stop running.
#[derive(Debug)]
pub(crate) struct Run;

impl CharacterSequenceHandler for Run {
    fn update(
        input: &CharacterInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> CharacterStatusUpdate {
        [AirborneCheck::update, RunStopCheck::update]
            .iter()
            .fold(None, |status_update, fn_update| {
                status_update.or_else(|| fn_update(input, character_status, kinematics))
            }).unwrap_or_else(|| CharacterStatusUpdate::default())
    }
}

#[cfg(test)]
mod test {
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterInput, CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics,
            ObjectStatus, ObjectStatusUpdate,
        },
    };

    use super::Run;
    use character::sequence_handler::CharacterSequenceHandler;

    #[test]
    fn jump_descend_when_airborne() {
        assert_eq!(
            CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            },
            Run::update(
                &CharacterInput::default(),
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Run,
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
