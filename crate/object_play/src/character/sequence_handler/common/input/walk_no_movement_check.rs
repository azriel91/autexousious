use object_model::config::object::CharacterSequenceId;

use character::sequence_handler::CharacterSequenceHandler;
use CharacterSequenceUpdateComponents;

/// Determines whether to switch to the `Stand` sequence based on X and Z input.
///
/// This should only be called from the Walk sequence handler.
#[derive(Debug)]
pub(crate) struct WalkNoMovementCheck;

impl CharacterSequenceHandler for WalkNoMovementCheck {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        if components.controller_input.x_axis_value == 0.
            && components.controller_input.z_axis_value == 0.
        {
            Some(CharacterSequenceId::Stand)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterStatus, Grounding, Mirrored, Position, RunCounter, SequenceStatus, Velocity,
        },
    };

    use super::WalkNoMovementCheck;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn stand_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Stand),
            WalkNoMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &CharacterSequenceId::Walk,
                &SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                &Mirrored::default(),
                &Grounding::default(),
                &RunCounter::default()
            ))
        );
    }

    #[test]
    fn none_when_x_axis_non_zero() {
        vec![1., -1.].into_iter().for_each(|x_input| {
            let input = ControllerInput::new(x_input, 0., false, false, false, false);

            assert_eq!(
                None,
                WalkNoMovementCheck::update(CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &CharacterSequenceId::Walk,
                    &SequenceStatus::default(),
                    &Position::default(),
                    &Velocity::default(),
                    &Mirrored::default(),
                    &Grounding::default(),
                    &RunCounter::default()
                ))
            );
        });
    }

    #[test]
    fn none_when_z_axis_non_zero() {
        vec![1., -1.].into_iter().for_each(|z_input| {
            let input = ControllerInput::new(0., z_input, false, false, false, false);

            assert_eq!(
                None,
                WalkNoMovementCheck::update(CharacterSequenceUpdateComponents::new(
                    &input,
                    &CharacterStatus::default(),
                    &CharacterSequenceId::Walk,
                    &SequenceStatus::default(),
                    &Position::default(),
                    &Velocity::default(),
                    &Mirrored::default(),
                    &Grounding::default(),
                    &RunCounter::default()
                ))
            );
        });
    }
}
