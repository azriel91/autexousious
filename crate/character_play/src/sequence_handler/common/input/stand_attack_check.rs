use character_model::config::CharacterSequenceId;

use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

/// Determines whether to switch to the `StandAttack` sequence based on Attack input.
#[derive(Debug)]
pub(crate) struct StandAttackCheck;

impl CharacterSequenceHandler for StandAttackCheck {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        if components.controller_input.attack {
            Some(CharacterSequenceId::StandAttack)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use character_model::{config::CharacterSequenceId, play::RunCounter};
    use game_input::ControllerInput;
    use object_model::entity::{Grounding, HealthPoints, Mirrored, Position, Velocity};
    use sequence_model::entity::SequenceStatus;

    use super::StandAttackCheck;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn no_change_when_no_attack_input() {
        let input = ControllerInput::default();

        assert_eq!(
            None,
            StandAttackCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
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
    fn walk_when_attack_is_true() {
        let mut input = ControllerInput::default();
        input.attack = true;

        assert_eq!(
            Some(CharacterSequenceId::StandAttack),
            StandAttackCheck::update(CharacterSequenceUpdateComponents::new(
                &input,
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
