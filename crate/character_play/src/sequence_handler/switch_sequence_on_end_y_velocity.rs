use character_model::config::CharacterSequenceId;
use derive_new::new;
use sequence_model::entity::SequenceStatus;

use crate::CharacterSequenceUpdateComponents;

#[derive(Debug, new)]
pub(crate) struct SwitchSequenceOnEndYVelocity {
    /// The sequence to switch to if Y velocity is upwards.
    pub upwards: CharacterSequenceId,
    /// The sequence to switch to if Y velocity is downwards.
    pub downwards: CharacterSequenceId,
}

impl SwitchSequenceOnEndYVelocity {
    pub fn update(
        &self,
        components: CharacterSequenceUpdateComponents<'_>,
    ) -> Option<CharacterSequenceId> {
        if components.sequence_status == SequenceStatus::End {
            if components.velocity[1] > 0. {
                Some(self.upwards)
            } else {
                Some(self.downwards)
            }
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

    use super::SwitchSequenceOnEndYVelocity;
    use crate::sequence_handler::CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        assert_eq!(
            None,
            SwitchSequenceOnEndYVelocity::new(
                CharacterSequenceId::DashForwardAscend,
                CharacterSequenceId::DashForwardDescend
            )
            .update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceId::DashForward,
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
    fn switches_to_upwards_when_sequence_ended_and_velocity_positive() {
        let mut velocity = Velocity::default();
        velocity[1] = 1.;

        assert_eq!(
            Some(CharacterSequenceId::DashForwardAscend),
            SwitchSequenceOnEndYVelocity::new(
                CharacterSequenceId::DashForwardAscend,
                CharacterSequenceId::DashForwardDescend
            )
            .update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceId::DashForward,
                SequenceStatus::End,
                &Position::default(),
                &velocity,
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }
    #[test]
    fn switches_to_downwards_when_sequence_ended_and_velocity_negative() {
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            Some(CharacterSequenceId::DashForwardDescend),
            SwitchSequenceOnEndYVelocity::new(
                CharacterSequenceId::DashForwardAscend,
                CharacterSequenceId::DashForwardDescend
            )
            .update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceId::DashForward,
                SequenceStatus::End,
                &Position::default(),
                &velocity,
                Mirrored::default(),
                Grounding::default(),
                RunCounter::default()
            ))
        );
    }
}
