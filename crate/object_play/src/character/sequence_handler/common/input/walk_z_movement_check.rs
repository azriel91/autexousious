use game_input::ControllerInput;
use object_model::entity::{CharacterStatus, CharacterStatusUpdate, Kinematics};

use character::sequence_handler::{common::SequenceRepeat, SequenceHandler};

/// Determines whether to switch to the `Stand` sequence based on Z input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub(crate) struct WalkZMovementCheck;

impl SequenceHandler for WalkZMovementCheck {
    fn update(
        input: &ControllerInput,
        character_status: &CharacterStatus,
        kinematics: &Kinematics<f32>,
    ) -> Option<CharacterStatusUpdate> {
        if input.z_axis_value != 0. {
            SequenceRepeat::update(input, character_status, kinematics)
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
            CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatus, ObjectStatusUpdate,
            RunCounter,
        },
    };

    use super::WalkZMovementCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn none_when_no_z_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            WalkZMovementCheck::update(
                &input,
                &CharacterStatus {
                    object_status: ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &Kinematics::default()
            )
        );
    }

    #[test]
    fn no_change_when_z_axis_non_zero() {
        vec![1., -1.].into_iter().for_each(|z_input| {
            let input = ControllerInput::new(0., z_input, false, false, false, false);

            assert_eq!(
                None,
                WalkZMovementCheck::update(
                    &input,
                    &CharacterStatus {
                        object_status: ObjectStatus {
                            sequence_id: CharacterSequenceId::Walk,
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
    fn restarts_walk_when_sequence_ended() {
        vec![1., -1.].into_iter().for_each(|z_input| {
            let input = ControllerInput::new(0., z_input, false, false, false, false);

            assert_eq!(
                Some(CharacterStatusUpdate {
                    object_status: ObjectStatusUpdate {
                        sequence_id: Some(CharacterSequenceId::Walk),
                        sequence_state: Some(SequenceState::Begin),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                WalkZMovementCheck::update(
                    &input,
                    &CharacterStatus {
                        run_counter: RunCounter::Increase(1),
                        object_status: ObjectStatus {
                            sequence_id: CharacterSequenceId::Walk,
                            sequence_state: SequenceState::End,
                            mirrored: false,
                            ..Default::default()
                        }
                    },
                    &Kinematics::default()
                )
            );
        });
    }
}
