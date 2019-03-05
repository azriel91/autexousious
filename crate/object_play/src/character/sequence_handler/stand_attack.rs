use character_model::config::CharacterSequenceId;
use sequence_model::entity::SequenceStatus;

use crate::{
    character::sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents,
};

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
    use character_model::config::CharacterSequenceId;
    use game_input::ControllerInput;
    use object_model::entity::{Grounding, HealthPoints, Mirrored, Position, RunCounter, Velocity};
    use sequence_model::entity::SequenceStatus;

    use super::StandAttack;
    use crate::{
        character::sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents,
    };

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
