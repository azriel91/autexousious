use character_model::config::CharacterSequenceName;

use crate::{
    sequence_handler::{
        CharacterSequenceHandler, SwitchSequenceOnEndYVelocity, SwitchSequenceOnLand,
    },
    CharacterSequenceUpdateComponents,
};

/// Disallow dash when landing in the middle of an attack.
const DASH_ATTACK_NOT_END: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceName::DashDescendLand);

/// Switch to the appropriate sequence based on velocity after attacking.
const DASH_ATTACK_END: SwitchSequenceOnEndYVelocity = SwitchSequenceOnEndYVelocity {
    upwards: CharacterSequenceName::DashForwardAscend,
    downwards: CharacterSequenceName::DashForwardDescend,
};

#[derive(Debug)]
pub(crate) struct DashAttack;

impl CharacterSequenceHandler for DashAttack {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceName> {
        DASH_ATTACK_NOT_END
            .update(components)
            .or_else(|| DASH_ATTACK_END.update(components))
    }
}

#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use super::DashAttack;
    use crate::sequence_handler::{CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn no_update_when_sequence_not_ended_and_not_on_ground() {
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            None,
            DashAttack::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::DashAttack,
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
    fn switches_to_land_when_on_ground() {
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            Some(CharacterSequenceName::DashDescendLand),
            DashAttack::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::FallForwardDescend,
                SequenceStatus::default(),
                &Position::default(),
                &velocity,
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
            DashAttack::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::DashAttack,
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
    fn switches_to_downwards_when_sequence_ended_and_velocity_negative() {
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            Some(CharacterSequenceName::DashForwardDescend),
            DashAttack::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::DashAttack,
                SequenceStatus::End,
                &Position::default(),
                &velocity,
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }
}
