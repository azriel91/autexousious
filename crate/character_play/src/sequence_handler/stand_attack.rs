use character_model::config::CharacterSequenceId;
use sequence_model::play::SequenceStatus;

use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

#[derive(Debug)]
pub(crate) struct StandAttack;

impl CharacterSequenceHandler for StandAttack {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        if components.sequence_status == SequenceStatus::End {
            Some(CharacterSequenceId::Stand)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceId, play::RunCounter};
    use game_input::ControllerInput;
    use object_model::play::{Grounding, HealthPoints, Mirrored, Position, Velocity};
    use sequence_model::play::SequenceStatus;

    use super::StandAttack;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            StandAttack::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::StandAttack,
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
    fn reverts_to_stand_when_sequence_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Stand),
            StandAttack::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::StandAttack,
                SequenceStatus::End,
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }
}
