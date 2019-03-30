use character_model::config::CharacterSequenceId;

use crate::{
    sequence_handler::{CharacterSequenceHandler, SequenceHandlerUtil},
    CharacterSequenceUpdateComponents,
};

/// Returns a `DashForward` update if forward and jump are pressed.
#[derive(Debug)]
pub(crate) struct DashForwardCheck;

impl CharacterSequenceHandler for DashForwardCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        // TODO: Don't handle action buttons in `CharacterSequenceHandler`s. Instead, each sequence
        // has default sequence update IDs for each action button, which are overridden by
        // configuration.
        let controller_input = &components.controller_input;
        if SequenceHandlerUtil::input_matches_direction(controller_input, components.mirrored)
            && controller_input.jump
        {
            Some(CharacterSequenceId::DashForward)
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

    use super::DashForwardCheck;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn returns_none_when_jump_is_not_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = false;
        assert_eq!(
            None,
            DashForwardCheck::update(CharacterSequenceUpdateComponents::new(
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
    fn returns_none_when_only_jump_is_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = true;
        assert_eq!(
            None,
            DashForwardCheck::update(CharacterSequenceUpdateComponents::new(
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
    fn switches_to_dash_forward_when_forward_and_jump_are_pressed() {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.0;
        controller_input.jump = true;
        assert_eq!(
            Some(CharacterSequenceId::DashForward),
            DashForwardCheck::update(CharacterSequenceUpdateComponents::new(
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
