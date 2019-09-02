use character_model::config::CharacterSequenceName;
use derive_new::new;
use sequence_model::play::SequenceStatus;

use crate::CharacterSequenceUpdateComponents;

#[derive(Debug, new)]
pub(crate) struct SwitchSequenceOnEndYVelocity {
    /// The sequence to switch to if Y velocity is upwards.
    pub upwards: CharacterSequenceName,
    /// The sequence to switch to if Y velocity is downwards.
    pub downwards: CharacterSequenceName,
}

impl SwitchSequenceOnEndYVelocity {
    pub fn update(
        &self,
        components: CharacterSequenceUpdateComponents<'_>,
    ) -> Option<CharacterSequenceName> {
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
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use super::SwitchSequenceOnEndYVelocity;
    use crate::sequence_handler::CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        assert_eq!(
            None,
            SwitchSequenceOnEndYVelocity::new(
                CharacterSequenceName::DashForwardAscend,
                CharacterSequenceName::DashForwardDescend
            )
            .update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::DashForward,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_upwards_when_sequence_ended_and_velocity_positive() {
        let mut velocity = Velocity::default();
        velocity[1] = 1.;

        assert_eq!(
            Some(CharacterSequenceName::DashForwardAscend),
            SwitchSequenceOnEndYVelocity::new(
                CharacterSequenceName::DashForwardAscend,
                CharacterSequenceName::DashForwardDescend
            )
            .update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::DashForward,
                SequenceStatus::End,
                &Position::default(),
                &velocity,
                Mirrored::default(),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }
    #[test]
    fn switches_to_downwards_when_sequence_ended_and_velocity_negative() {
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            Some(CharacterSequenceName::DashForwardDescend),
            SwitchSequenceOnEndYVelocity::new(
                CharacterSequenceName::DashForwardAscend,
                CharacterSequenceName::DashForwardDescend
            )
            .update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::DashForward,
                SequenceStatus::End,
                &Position::default(),
                &velocity,
                Mirrored::default(),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }
}
