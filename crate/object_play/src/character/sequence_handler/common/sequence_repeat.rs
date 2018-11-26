use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::ObjectStatusUpdate,
};

use character::sequence_handler::SequenceHandler;
use CharacterSequenceUpdateComponents;

/// Restarts a sequence when it has reached the end.
#[derive(Debug)]
pub(crate) struct SequenceRepeat;

impl SequenceHandler for SequenceRepeat {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<ObjectStatusUpdate<CharacterSequenceId>> {
        if components.object_status.sequence_state == SequenceState::End {
            let sequence_id = Some(components.object_status.sequence_id);
            let sequence_state = Some(SequenceState::Begin);
            let mirrored = None;
            let grounding = None;

            Some(ObjectStatusUpdate::new(
                sequence_id,
                sequence_state,
                mirrored,
                grounding,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{CharacterStatus, Kinematics, ObjectStatus, ObjectStatusUpdate, RunCounter},
    };

    use super::SequenceRepeat;
    use character::sequence_handler::SequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_change_when_sequence_begin() {
        assert_eq!(
            None,
            SequenceRepeat::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    sequence_state: SequenceState::Begin,
                    ..Default::default()
                },
                &Kinematics::default(),
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
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    sequence_state: SequenceState::Ongoing,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn restarts_sequence_when_no_input_and_sequence_end() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Walk),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            }),
            SequenceRepeat::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::Walk,
                    sequence_state: SequenceState::End,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            ))
        );
    }
}
