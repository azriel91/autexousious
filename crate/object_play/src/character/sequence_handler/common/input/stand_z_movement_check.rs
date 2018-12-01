use object_model::config::object::CharacterSequenceId;

use character::sequence_handler::CharacterSequenceHandler;
use CharacterSequenceUpdateComponents;

/// Determines whether to switch to the `Walk` sequence based on Z input.
///
/// This should only be called from the Stand sequence handler.
#[derive(Debug)]
pub(crate) struct StandZMovementCheck;

impl CharacterSequenceHandler for StandZMovementCheck {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        if components.controller_input.z_axis_value != 0. {
            Some(CharacterSequenceId::Walk)
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

    use super::StandZMovementCheck;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_change_when_no_z_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            StandZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &CharacterSequenceId::default(),
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
    fn walk_when_z_axis_is_non_zero() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Walk),
            StandZMovementCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &CharacterSequenceId::default(),
                &SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                &Mirrored::default(),
                &Grounding::default(),
                &RunCounter::default()
            ))
        );
    }
}
