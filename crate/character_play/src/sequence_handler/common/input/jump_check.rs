use character_model::config::CharacterSequenceId;

use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

/// Returns a `Jump` update if jump is pressed.
#[derive(Debug)]
pub(crate) struct JumpCheck;

impl CharacterSequenceHandler for JumpCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        // TODO: Don't handle action buttons in `CharacterSequenceHandler`s. Instead, each sequence
        // has default sequence update IDs for each action button, which are overridden by
        // configuration.
        if components.controller_input.jump {
            Some(CharacterSequenceId::Jump)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use character_model::{config::CharacterSequenceId, play::RunCounter};
    use game_input::ControllerInput;
    use object_model::play::{Grounding, HealthPoints, Mirrored, Position, Velocity};
    use sequence_model::play::SequenceStatus;

    use super::JumpCheck;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn returns_none_when_jump_is_not_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = false;
        assert_eq!(
            None,
            JumpCheck::update(CharacterSequenceUpdateComponents::new(
                &controller_input,
                HealthPoints::default(),
                CharacterSequenceId::default(),
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
    fn switches_to_jump_when_jump_is_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = true;
        assert_eq!(
            Some(CharacterSequenceId::Jump),
            JumpCheck::update(CharacterSequenceUpdateComponents::new(
                &controller_input,
                HealthPoints::default(),
                CharacterSequenceId::default(),
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
