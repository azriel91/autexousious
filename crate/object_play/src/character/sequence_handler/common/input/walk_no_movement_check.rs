use game_input::ControllerInput;
use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::{CharacterStatus, Kinematics, ObjectStatus, ObjectStatusUpdate, RunCounter},
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
        _object_status: &ObjectStatus<CharacterSequenceId>,
        _kinematics: &Kinematics<f32>,
        _run_counter: RunCounter,
    ) -> Option<ObjectStatusUpdate<CharacterSequenceId>> {
        if input.x_axis_value == 0. && input.z_axis_value == 0. {
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Stand),
                sequence_state: Some(SequenceState::Begin),
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
        entity::{CharacterStatus, Kinematics, ObjectStatus, ObjectStatusUpdate, RunCounter},
    };

    use super::WalkNoMovementCheck;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn stand_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Stand),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            }),
            WalkNoMovementCheck::update(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
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
                    &CharacterStatus::default(),
                    &ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        ..Default::default()
                    },
                    &Kinematics::default(),
                    RunCounter::default()
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
                    &CharacterStatus::default(),
                    &ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        ..Default::default()
                    },
                    &Kinematics::default(),
                    RunCounter::default()
                )
            );
        });
    }
}
