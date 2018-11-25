use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{
        CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
        RunCounter,
    },
};

use character::sequence_handler::{
    common::{grounding::AirborneCheck, input::RunStopCheck, status::AliveCheck},
    CharacterSequenceHandler, SequenceHandler,
};

/// Hold forward to run, release to stop running.
#[derive(Debug)]
pub(crate) struct Run;

impl CharacterSequenceHandler for Run {
    fn update(
        input: &ControllerInput,
        character_status: &CharacterStatus,
        object_status: &ObjectStatus<CharacterSequenceId>,
        kinematics: &Kinematics<f32>,
        run_counter: RunCounter,
    ) -> (
        CharacterStatusUpdate,
        ObjectStatusUpdate<CharacterSequenceId>,
    ) {
        [
            AliveCheck::update,
            AirborneCheck::update,
            RunStopCheck::update,
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
        .unwrap_or_else(|| {
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::default(),
            )
        })
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, CharacterStatusUpdate, Grounding, Kinematics, ObjectStatus,
            ObjectStatusUpdate, RunCounter,
        },
    };

    use super::Run;
    use character::sequence_handler::CharacterSequenceHandler;

    #[test]
    fn jump_descend_when_airborne() {
        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::JumpDescend),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            Run::update(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
                    grounding: Grounding::Airborne,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn reverts_to_run_stop_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::RunStop),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            Run::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
                    mirrored: false,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn keeps_running_when_x_axis_positive_and_non_mirrored() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::default()
            ),
            Run::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
                    mirrored: false,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn keeps_running_when_x_axis_negative_and_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::default()
            ),
            Run::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
                    mirrored: true,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn restarts_run_when_sequence_ended() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    (
                        CharacterStatusUpdate::default(),
                        ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Run),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        }
                    ),
                    Run::update(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::Run,
                            sequence_state: SequenceState::End,
                            mirrored,
                            ..Default::default()
                        },
                        &Kinematics::default(),
                        RunCounter::default()
                    )
                );
            });
    }

    #[test]
    fn reverts_to_run_stop_when_x_axis_negative_and_non_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::RunStop),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            Run::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
                    mirrored: false,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn reverts_to_run_stop_when_x_axis_positive_and_mirrored() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::RunStop),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            ),
            Run::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
                    mirrored: true,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }

    #[test]
    fn keeps_running_when_x_axis_positive_z_axis_non_zero_and_non_mirrored() {
        let input = ControllerInput::new(1., 1., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::default()
            ),
            Run::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
                    mirrored: false,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );

        let input = ControllerInput::new(1., -1., false, false, false, false);

        assert_eq!(
            (
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate::default()
            ),
            Run::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
                    mirrored: false,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            )
        );
    }
}
