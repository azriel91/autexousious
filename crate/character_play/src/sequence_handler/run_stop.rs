use character_model::config::CharacterSequenceName;
use sequence_model::play::SequenceStatus;

use crate::{
    sequence_handler::{
        common::{grounding::AirborneCheck, status::AliveCheck},
        CharacterSequenceHandler,
    },
    CharacterSequenceUpdateComponents,
};

#[derive(Debug)]
pub(crate) struct RunStop;

impl CharacterSequenceHandler for RunStop {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        [AliveCheck::update, AirborneCheck::update]
            .iter()
            .fold(None, |status_update, fn_update| {
                status_update.or_else(|| fn_update(components))
            })
            .or_else(|| {
                if components.sequence_status == SequenceStatus::End {
                    Some(CharacterSequenceName::Stand)
                } else {
                    None
                }
            })
    }
}

#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use super::RunStop;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn jump_descend_when_airborne() {
        assert_eq!(
            Some(CharacterSequenceName::JumpDescend),
            RunStop::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                &CharacterSequenceName::RunStop,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            RunStop::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::RunStop,
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
            Some(CharacterSequenceName::Stand),
            RunStop::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::RunStop,
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
