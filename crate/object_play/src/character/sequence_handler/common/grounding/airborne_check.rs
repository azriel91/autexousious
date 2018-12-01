use object_model::{config::object::CharacterSequenceId, entity::Grounding};

use character::sequence_handler::CharacterSequenceHandler;
use CharacterSequenceUpdateComponents;

/// Returns a `JumpDescend` update if the grounding is `Airborne`.
#[derive(Debug)]
pub(crate) struct AirborneCheck;

impl CharacterSequenceHandler for AirborneCheck {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        if components.grounding == Grounding::Airborne {
            Some(CharacterSequenceId::JumpDescend)
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

    use super::AirborneCheck;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn returns_none_when_grounding_is_on_ground() {
        assert_eq!(
            None,
            AirborneCheck::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                CharacterSequenceId::Stand,
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
    fn switches_to_jump_descend_when_grounding_is_airborne() {
        assert_eq!(
            Some(CharacterSequenceId::JumpDescend),
            AirborneCheck::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                CharacterSequenceId::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }
}
