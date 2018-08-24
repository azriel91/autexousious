use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, CharacterStatusUpdate, Kinematics, ObjectStatusUpdate},
};

use character::sequence_handler::SequenceHandler;

/// Determines whether to switch to the `Stand` sequence based on X and Z input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub(crate) struct WalkNoMovementCheck;

impl SequenceHandler for WalkNoMovementCheck {
    fn update(
        input: &ControllerInput,
        _character_status: &CharacterStatus,
        _kinematics: &Kinematics<f32>,
    ) -> Option<CharacterStatusUpdate> {
        if input.x_axis_value == 0. && input.z_axis_value == 0. {
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Stand),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            })
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
        },
    };

    use super::WalkNoMovementCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn stand_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterStatusUpdate {
                object_status: ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Stand),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                },
                ..Default::default()
            }),
            WalkNoMovementCheck::update(
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
    fn none_when_x_axis_non_zero() {
        vec![1., -1.].into_iter().for_each(|x_input| {
            let input = ControllerInput::new(x_input, 0., false, false, false, false);

            assert_eq!(
                None,
                WalkNoMovementCheck::update(
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
    fn none_when_z_axis_non_zero() {
        vec![1., -1.].into_iter().for_each(|z_input| {
            let input = ControllerInput::new(0., z_input, false, false, false, false);

            assert_eq!(
                None,
                WalkNoMovementCheck::update(
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
}
