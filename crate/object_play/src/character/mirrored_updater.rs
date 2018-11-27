use game_input::ControllerInput;
use object_model::{
    config::object::CharacterSequenceId,
    entity::{Mirrored, ObjectStatus},
};

use character::sequence_handler::SequenceHandlerUtil;

/// Updates the `Mirrored` component for character entities.
#[derive(Debug)]
pub struct MirroredUpdater;

impl MirroredUpdater {
    /// Returns the updated `Mirrored` value.
    ///
    /// # Parameters
    ///
    /// * `controller_input`: Controller input for this character.
    /// * `object_status`: Current object status.
    /// * `mirrored`: Whether the object is mirrored (facing left).
    pub fn update(
        controller_input: &ControllerInput,
        object_status: &ObjectStatus<CharacterSequenceId>,
        mirrored: Mirrored,
    ) -> Mirrored {
        match object_status.sequence_id {
            CharacterSequenceId::Stand
            | CharacterSequenceId::Walk
            | CharacterSequenceId::JumpAscend
            | CharacterSequenceId::JumpDescend => {}
            _ => return mirrored,
        }

        if SequenceHandlerUtil::input_opposes_direction(controller_input, mirrored) {
            !mirrored
        } else {
            mirrored
        }
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{Mirrored, ObjectStatus},
    };

    use super::MirroredUpdater;

    #[test]
    fn no_change_when_no_input() {
        let input = ControllerInput::default();

        vec![true, false].into_iter().for_each(|mirrored| {
            verify_for_sequences(mirrored.into(), &input, mirrored.into());
        });
    }

    #[test]
    fn no_change_when_non_applicable_sequences() {
        let input = ControllerInput::default();

        vec![true, false].into_iter().for_each(|mirrored| {
            verify_for_sequences_na(&input, mirrored.into());
        });
    }

    #[test]
    fn true_when_input_is_left() {
        let mut input = ControllerInput::default();
        input.x_axis_value = -1.;

        vec![true, false].into_iter().for_each(|mirrored| {
            verify_for_sequences(Mirrored(true), &input, mirrored.into());
        });
    }

    #[test]
    fn false_when_input_is_right() {
        let mut input = ControllerInput::default();
        input.x_axis_value = 1.;

        vec![true, false].into_iter().for_each(|mirrored| {
            verify_for_sequences(Mirrored(false), &input, mirrored.into());
        });
    }

    fn verify_for_sequences(
        expected: Mirrored,
        controller_input: &ControllerInput,
        mirrored: Mirrored,
    ) {
        vec![
            CharacterSequenceId::Stand,
            CharacterSequenceId::Walk,
            CharacterSequenceId::JumpAscend,
            CharacterSequenceId::JumpDescend,
        ]
        .into_iter()
        .for_each(|sequence_id| {
            assert_eq!(
                expected,
                MirroredUpdater::update(
                    controller_input,
                    &ObjectStatus {
                        sequence_id,
                        ..Default::default()
                    },
                    mirrored,
                )
            );
        });
    }

    fn verify_for_sequences_na(controller_input: &ControllerInput, mirrored: Mirrored) {
        vec![CharacterSequenceId::Run, CharacterSequenceId::RunStop]
            .into_iter()
            .for_each(|sequence_id| {
                assert_eq!(
                    mirrored,
                    MirroredUpdater::update(
                        controller_input,
                        &ObjectStatus {
                            sequence_id,
                            ..Default::default()
                        },
                        mirrored,
                    )
                );
            });
    }
}
