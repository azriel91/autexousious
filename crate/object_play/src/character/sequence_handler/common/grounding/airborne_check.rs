use character_model::config::CharacterSequenceId;
use object_model::entity::Grounding;

use crate::{
    character::sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents,
};

/// Returns a `JumpDescend` update if the grounding is `Airborne`.
#[derive(Debug)]
pub(crate) struct AirborneCheck;

impl CharacterSequenceHandler for AirborneCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        if components.grounding == Grounding::Airborne {
            Some(CharacterSequenceId::JumpDescend)
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

    use super::AirborneCheck;
    use crate::{
        character::sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents,
    };

    #[test]
    fn returns_none_when_grounding_is_on_ground() {
        assert_eq!(
            None,
            AirborneCheck::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
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
                HealthPoints::default(),
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
