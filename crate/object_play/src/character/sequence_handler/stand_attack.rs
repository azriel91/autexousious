use object_model::{
    config::object::{CharacterSequenceId, SequenceState},
    entity::ObjectStatusUpdate,
};

use character::sequence_handler::CharacterSequenceHandler;
use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct StandAttack;

impl CharacterSequenceHandler for StandAttack {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        let mut object_status_update = ObjectStatusUpdate::default();
        if components.object_status.sequence_state == SequenceState::End {
            object_status_update.sequence_id = Some(CharacterSequenceId::Stand);
            object_status_update.sequence_state = Some(SequenceState::Begin);
        }

        object_status_update
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::{CharacterSequenceId, SequenceState},
        entity::{CharacterStatus, Kinematics, ObjectStatus, ObjectStatusUpdate, RunCounter},
    };

    use super::StandAttack;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate::default(),
            StandAttack::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::StandAttack,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn reverts_to_stand_when_sequence_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Stand),
                sequence_state: Some(SequenceState::Begin),
                ..Default::default()
            },
            StandAttack::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::StandAttack,
                    sequence_state: SequenceState::End,
                    ..Default::default()
                },
                &Kinematics::default(),
                RunCounter::default()
            ))
        );
    }
}
