use character_model::config::CharacterSequenceId;

use crate::{
    character::sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents,
};

/// Returns a `Dodge` update if defend is pressed.
#[derive(Debug)]
pub(crate) struct DodgeCheck;

impl CharacterSequenceHandler for DodgeCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        // TODO: Don't handle action buttons in `CharacterSequenceHandler`s. Instead, each sequence
        // has default sequence update IDs for each action button, which are overridden by
        // configuration.
        if components.controller_input.defend {
            Some(CharacterSequenceId::Dodge)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use character_model::config::CharacterSequenceId;
    use game_input::ControllerInput;
    use object_model::entity::{Grounding, HealthPoints, Mirrored, Position, RunCounter, Velocity};
    use sequence_model::entity::SequenceStatus;

    use super::DodgeCheck;
    use crate::{
        character::sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents,
    };

    #[test]
    fn returns_none_when_defend_is_not_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.defend = false;
        assert_eq!(
            None,
            DodgeCheck::update(CharacterSequenceUpdateComponents::new(
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
    fn switches_to_dodge_when_defend_is_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.defend = true;
        assert_eq!(
            Some(CharacterSequenceId::Dodge),
            DodgeCheck::update(CharacterSequenceUpdateComponents::new(
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
