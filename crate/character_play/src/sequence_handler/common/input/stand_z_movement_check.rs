use character_model::config::CharacterSequenceName;

use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

/// Determines whether to switch to the `Walk` sequence based on Z input.
///
/// This should only be called from the Stand sequence handler.
#[derive(Debug)]
pub(crate) struct StandZMovementCheck;

impl CharacterSequenceHandler for StandZMovementCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.controller_input.z_axis_value != 0. {
            Some(CharacterSequenceName::Walk)
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

    use super::StandZMovementCheck;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn no_change_when_no_z_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            StandZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::default(),
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
    fn walk_when_z_axis_is_non_zero() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            StandZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::default(),
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }
}
