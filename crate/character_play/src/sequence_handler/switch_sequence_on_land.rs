use character_model::config::CharacterSequenceName;
use object_model::play::Grounding;

use crate::CharacterSequenceUpdateComponents;

#[derive(Debug)]
pub(crate) struct SwitchSequenceOnLand(
    /// The sequence to switch to.
    pub CharacterSequenceName,
);

impl SwitchSequenceOnLand {
    pub fn update<'c>(
        &self,
        components: CharacterSequenceUpdateComponents<'c>,
    ) -> Option<CharacterSequenceName> {
        if components.grounding == Grounding::OnGround {
            Some(self.0.clone())
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

    use super::SwitchSequenceOnLand;
    use crate::CharacterSequenceUpdateComponents;

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            None,
            SwitchSequenceOnLand(CharacterSequenceName::FallForwardLand).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    HealthPoints::default(),
                    &CharacterSequenceName::FallForwardDescend,
                    SequenceStatus::default(),
                    &Position::default(),
                    &velocity,
                    Mirrored::default(),
                    Grounding::Airborne,
                    RunCounter::default()
                )
            )
        );
    }

    #[test]
    fn switches_to_land_when_on_ground() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            Some(CharacterSequenceName::FallForwardLand),
            SwitchSequenceOnLand(CharacterSequenceName::FallForwardLand).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    HealthPoints::default(),
                    &CharacterSequenceName::FallForwardDescend,
                    SequenceStatus::default(),
                    &Position::default(),
                    &velocity,
                    Mirrored::default(),
                    Grounding::OnGround,
                    RunCounter::default()
                )
            )
        );
    }
}
