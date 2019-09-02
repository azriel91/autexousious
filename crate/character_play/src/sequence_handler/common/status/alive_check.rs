use character_model::config::CharacterSequenceName;

use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

/// Returns the appropriate falling sequence if HP is 0.
#[derive(Debug)]
pub(crate) struct AliveCheck;

impl CharacterSequenceHandler for AliveCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.health_points == 0 {
            Some(CharacterSequenceName::FallForwardDescend)
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

    use super::AliveCheck;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn returns_none_when_hp_is_above_zero() {
        assert_eq!(
            None,
            AliveCheck::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_fall_forward_descend_when_hp_is_zero() {
        assert_eq!(
            Some(CharacterSequenceName::FallForwardDescend),
            AliveCheck::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints(0),
                CharacterSequenceName::Stand,
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
