use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{common::SequenceRepeat, CharacterSequenceHandler},
    CharacterSequenceUpdateComponents,
};

/// Determines whether to switch to the `Stand` sequence based on Z input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub(crate) struct WalkZMovementCheck;

impl CharacterSequenceHandler for WalkZMovementCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.controller_input.z_axis_value != 0. {
            SequenceRepeat::update(components)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use super::WalkZMovementCheck;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn none_when_no_z_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            WalkZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
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
                    HealthPoints::default(),
                    &CharacterSequenceName::Walk,
                    SequenceStatus::default(),
                    &Position::default(),
                    &Velocity::default(),
                    Mirrored::default(),
                    Grounding::default(),
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
                Some(CharacterSequenceName::Walk),
                WalkZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                    &input,
                    HealthPoints::default(),
                    &CharacterSequenceName::Walk,
                    SequenceStatus::End,
                    &Position::default(),
                    &Velocity::default(),
                    Mirrored(false),
                    Grounding::default(),
                    RunCounter::Increase(1)
                ))
            );
        });
    }
}
