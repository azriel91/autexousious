use object_model::{config::object::CharacterSequenceId, entity::SequenceStatus};

use character::sequence_handler::CharacterSequenceHandler;
use CharacterSequenceUpdateComponents;

/// Restarts a sequence when it has reached the end.
#[derive(Debug)]
pub(crate) struct SequenceRepeat;

impl CharacterSequenceHandler for SequenceRepeat {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        if components.sequence_status == SequenceStatus::End {
            Some(components.character_sequence_id)
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
        entity::{CharacterStatus, Grounding, Kinematics, Mirrored, RunCounter, SequenceStatus},
    };

    use super::SequenceRepeat;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_change_when_sequence_begin() {
        assert_eq!(
            None,
            SequenceRepeat::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                CharacterSequenceId::Walk,
                SequenceStatus::Begin,
                &Kinematics::default(),
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
                &CharacterStatus::default(),
                CharacterSequenceId::Walk,
                SequenceStatus::Ongoing,
                &Kinematics::default(),
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
            Some(CharacterSequenceId::Walk),
            SequenceRepeat::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                CharacterSequenceId::Walk,
                SequenceStatus::End,
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }
}
