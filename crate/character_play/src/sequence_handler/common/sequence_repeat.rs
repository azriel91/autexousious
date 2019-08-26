use character_model::config::CharacterSequenceName;
use sequence_model::play::SequenceStatus;

use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

/// Restarts a sequence when it has reached the end.
#[derive(Debug)]
pub(crate) struct SequenceRepeat;

impl CharacterSequenceHandler for SequenceRepeat {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        if components.sequence_status == SequenceStatus::End {
            Some(components.character_sequence_name.clone())
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

    use super::SequenceRepeat;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn no_change_when_sequence_begin() {
        assert_eq!(
            None,
            SequenceRepeat::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                &CharacterSequenceName::Walk,
                SequenceStatus::Begin,
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn no_change_when_sequence_ongoing() {
        assert_eq!(
            None,
            SequenceRepeat::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                &CharacterSequenceName::Walk,
                SequenceStatus::Ongoing,
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn restarts_sequence_when_no_input_and_sequence_end() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            SequenceRepeat::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                &CharacterSequenceName::Walk,
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
