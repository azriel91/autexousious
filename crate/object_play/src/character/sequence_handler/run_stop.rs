use object_model::{
    config::object::CharacterSequenceId,
    entity::{ObjectStatusUpdate, SequenceStatus},
};

use character::sequence_handler::{
    common::{grounding::AirborneCheck, status::AliveCheck},
    CharacterSequenceHandler, SequenceHandler,
};
use CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct RunStop;

impl CharacterSequenceHandler for RunStop {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> ObjectStatusUpdate<CharacterSequenceId> {
        [AliveCheck::update, AirborneCheck::update]
            .iter()
            .fold(None, |status_update, fn_update| {
                status_update.or_else(|| fn_update(components))
            })
            .unwrap_or_else(|| {
                let mut object_status_update = ObjectStatusUpdate::default();
                if components.sequence_status == SequenceStatus::End {
                    object_status_update.sequence_id = Some(CharacterSequenceId::Stand);
                }
                object_status_update
            })
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterStatus, Grounding, Kinematics, Mirrored, ObjectStatus, ObjectStatusUpdate,
            RunCounter, SequenceStatus,
        },
    };

    use super::RunStop;
    use character::sequence_handler::CharacterSequenceHandler;
    use CharacterSequenceUpdateComponents;

    #[test]
    fn jump_descend_when_airborne() {
        assert_eq!(
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::JumpDescend),
            },
            RunStop::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::RunStop,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
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
            ObjectStatusUpdate::default(),
            RunStop::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::RunStop,
                },
                SequenceStatus::default(),
                &Kinematics::default(),
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
            ObjectStatusUpdate {
                sequence_id: Some(CharacterSequenceId::Stand),
            },
            RunStop::update(CharacterSequenceUpdateComponents::new(
                &input,
                &CharacterStatus::default(),
                &ObjectStatus {
                    sequence_id: CharacterSequenceId::RunStop,
                },
                SequenceStatus::End,
                &Kinematics::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }
}
