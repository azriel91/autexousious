use object_model::{config::object::CharacterSequenceId, entity::SequenceStatus};

use crate::character::sequence_handler::{
    common::{grounding::AirborneCheck, status::AliveCheck},
    CharacterSequenceHandler,
};
use crate::CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct RunStop;

impl CharacterSequenceHandler for RunStop {
    fn update<'c>(
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceId> {
        [AliveCheck::update, AirborneCheck::update]
            .iter()
            .fold(None, |status_update, fn_update| {
                status_update.or_else(|| fn_update(components))
            })
            .or_else(|| {
                if components.sequence_status == SequenceStatus::End {
                    Some(CharacterSequenceId::Stand)
                } else {
                    None
                }
            })
    }
}

#[cfg(test)]
mod test {
    use game_input::ControllerInput;
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            Grounding, HealthPoints, Mirrored, Position, RunCounter, SequenceStatus, Velocity,
        },
    };

    use super::RunStop;
    use crate::character::sequence_handler::CharacterSequenceHandler;
    use crate::CharacterSequenceUpdateComponents;

    #[test]
    fn jump_descend_when_airborne() {
        assert_eq!(
            Some(CharacterSequenceId::JumpDescend),
            RunStop::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceId::RunStop,
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
                CharacterSequenceId::RunStop,
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
            RunStop::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::RunStop,
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
