use object_model::{config::object::CharacterSequenceId, entity::ObjectStatusUpdate};

use character::sequence_handler::{common::SequenceRepeat, SequenceHandler};
use CharacterSequenceUpdateComponents;

/// Determines whether to switch to the `Stand` sequence based on Z input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub(crate) struct WalkZMovementCheck;

impl SequenceHandler for WalkZMovementCheck {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<ObjectStatusUpdate<CharacterSequenceId>> {
        if components.controller_input.z_axis_value != 0. {
            SequenceRepeat::update(components)
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
            CharacterStatus, Kinematics, Mirrored, ObjectStatus, ObjectStatusUpdate, RunCounter,
        },
    };

    use super::WalkZMovementCheck;
    use character::sequence_handler::SequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn none_when_no_z_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            WalkZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn no_change_when_z_axis_non_zero() {
        vec![1., -1.].into_iter().for_each(|z_input| {
            let input = ControllerInput::new(0., z_input, false, false, false, false);

            assert_eq!(
                None,
                WalkZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        ..Default::default()
                    },
                    &Kinematics::default(),
                    RunCounter::default()
                ))
            );
        });
    }

    #[test]
    fn restarts_walk_when_sequence_ended() {
        vec![1., -1.].into_iter().for_each(|z_input| {
            let input = ControllerInput::new(0., z_input, false, false, false, false);

            assert_eq!(
                Some(ObjectStatusUpdate {
                    sequence_id: Some(CharacterSequenceId::Walk),
                    sequence_state: Some(SequenceState::Begin),
                    ..Default::default()
                }),
                WalkZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &ObjectStatus {
                        sequence_id: CharacterSequenceId::Walk,
                        sequence_state: SequenceState::End,
                        mirrored: Mirrored(false),
                        ..Default::default()
                    },
                    &Kinematics::default(),
                    RunCounter::Increase(1)
                ))
            );
        });
    }
}
