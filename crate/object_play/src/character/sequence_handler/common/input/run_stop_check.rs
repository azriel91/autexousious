use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{
        CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
    },
};

use character::sequence_handler::{common::SequenceRepeat, SequenceHandler, SequenceHandlerUtil};

/// Determines whether to switch to the `RunStop` sequence based on X input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub(crate) struct RunStopCheck;

impl SequenceHandler for RunStopCheck {
    fn update(
        input: &ControllerInput,
        character_status: &CharacterStatus,
        object_status: &ObjectStatus<CharacterSequenceId>,
        kinematics: &Kinematics<f32>,
    ) -> Option<(
        CharacterStatusUpdate,
        ObjectStatusUpdate<CharacterSequenceId>,
    )> {
        if SequenceHandlerUtil::input_matches_direction(input, object_status.mirrored) {
            SequenceRepeat::update(input, character_status, object_status, kinematics)
        } else {
            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::RunStop),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{
            CharacterStatus, CharacterStatusUpdate, HealthPoints, Kinematics, ObjectStatus,
            ObjectStatusUpdate, RunCounter,
        },
    };

    use super::RunStopCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn none_when_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    None,
                    RunStopCheck::update(
                        &input,
                        &CharacterStatus {
                            hp: HealthPoints(100),
                            ..Default::default()
                        },
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::Walk,
                            mirrored,
                            ..Default::default()
                        },
                        &Kinematics::default()
                    )
                );
            });
    }

    #[test]
    fn run_stop_when_no_x_input() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            Some((
                CharacterStatusUpdate::default(),
                ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::RunStop),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }
            )),
            RunStopCheck::update(
                &input,
                &CharacterStatus {
                    hp: HealthPoints(100),
                    ..Default::default()
                },
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    mirrored: false,
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn run_stop_when_input_different_direction() {
        vec![(1., true), (-1., false)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some((
                        CharacterStatusUpdate::default(),
                        ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::RunStop),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        }
                    )),
                    RunStopCheck::update(
                        &input,
                        &CharacterStatus {
                            hp: HealthPoints(100),
                            ..Default::default()
                        },
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::Walk,
                            mirrored,
                            ..Default::default()
                        },
                        &Kinematics::default()
                    )
                );
            });
    }

    #[test]
    fn restarts_run_when_sequence_ended() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some((
                        CharacterStatusUpdate::default(),
                        ObjectStatusUpdate {
                            sequence_id: Some(CharacterSequenceId::Run),
                            sequence_state: Some(SequenceState::Begin),
                            ..Default::default()
                        }
                    )),
                    RunStopCheck::update(
                        &input,
                        &CharacterStatus {
                            run_counter: RunCounter::Increase(1),
                            hp: HealthPoints(100)
                        },
                        &ObjectStatus {
                            sequence_id: CharacterSequenceId::Run,
                            sequence_state: SequenceState::End,
                            mirrored,
                            ..Default::default()
                        },
                        &Kinematics::default()
                    )
                );
            });
    }
}
