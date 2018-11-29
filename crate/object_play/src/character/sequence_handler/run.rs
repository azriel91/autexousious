use object_model::{config::object::CharacterSequenceId, entity::ObjectStatusUpdate};

use character::sequence_handler::{
    common::{grounding::AirborneCheck, input::RunStopCheck, status::AliveCheck},
    CharacterSequenceHandler, SequenceHandler,
};
use CharacterSequenceUpdateComponents;

/// Hold forward to run, release to stop running.
#[derive(Debug)]
pub(crate) struct Run;

impl CharacterSequenceHandler for Run {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        [
            AliveCheck::update,
            AirborneCheck::update,
            RunStopCheck::update,
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

    use super::Run;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn jump_descend_when_airborne() {
        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::JumpDescend),
            },
            Run::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn reverts_to_run_stop_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::RunStop),
            },
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
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
    fn keeps_running_when_x_axis_positive_and_non_mirrored() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate::default(),
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
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
    fn keeps_running_when_x_axis_negative_and_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate::default(),
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
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
    fn restarts_run_when_sequence_ended() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    ObjectStatusUpdate {
                        sequence_id: Some(CharacterSequenceId::Run),
                    },
                    Run::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        &CharacterStatus::default(),
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::Run,
                        },
                        SequenceStatus::End,
                        &Kinematics::default(),
                        mirrored.into(),
                        Grounding::default(),
                        RunCounter::default()
                    ))
                );
            });
    }

    #[test]
    fn reverts_to_run_stop_when_x_axis_negative_and_non_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::RunStop),
            },
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
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
    fn reverts_to_run_stop_when_x_axis_positive_and_mirrored() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::RunStop),
            },
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
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
    fn keeps_running_when_x_axis_positive_z_axis_non_zero_and_non_mirrored() {
        let input = ControllerInput::new(1., 1., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate::default(),
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::default()
            ))
        );

        let input = ControllerInput::new(1., -1., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate::default(),
            Run::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Run,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }
}
