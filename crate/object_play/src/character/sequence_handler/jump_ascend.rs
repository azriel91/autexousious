use character_model::config::CharacterSequenceId;
use sequence_model::entity::SequenceStatus;

use crate::{
    character::sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents,
};

#[derive(Debug)]
pub(crate) struct JumpAscend;

impl CharacterSequenceHandler for JumpAscend {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        // Switch to jump_descend when Y axis velocity is no longer upwards.
        if components.velocity[1] <= 0. {
            Some(CharacterSequenceId::JumpDescend)
        } else if components.sequence_status == SequenceStatus::End {
            Some(CharacterSequenceId::JumpAscend)
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

    use super::JumpAscend;
    use crate::{
        character::sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents,
    };

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut velocity = Velocity::default();
        velocity[1] = 1.;

        assert_eq!(
            None,
            JumpAscend::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::JumpAscend,
                SequenceStatus::default(),
                &Position::default(),
                &velocity,
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn restarts_jump_ascend_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut velocity = Velocity::default();
        velocity[1] = 1.;

        assert_eq!(
            Some(CharacterSequenceId::JumpAscend),
            JumpAscend::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::JumpAscend,
                SequenceStatus::End,
                &Position::default(),
                &velocity,
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_jump_descend_when_y_velocity_is_zero_or_downwards() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut downwards_velocity = Velocity::default();
        downwards_velocity[1] = -1.;

        vec![Velocity::default(), downwards_velocity]
            .into_iter()
            .for_each(|velocity| {
                assert_eq!(
                    Some(CharacterSequenceId::JumpDescend),
                    JumpAscend::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        CharacterSequenceId::JumpAscend,
                        SequenceStatus::Ongoing,
                        &Position::default(),
                        &velocity,
                        Mirrored::default(),
                        Grounding::Airborne,
                        RunCounter::default()
                    ))
                );
            });
    }
}
