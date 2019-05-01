use character_model::config::CharacterSequenceId;

use crate::{
    sequence_handler::{
        CharacterSequenceHandler, SwitchSequenceOnEndYVelocity, SwitchSequenceOnLand,
    },
    CharacterSequenceUpdateComponents,
};

/// Disallow dash when landing in the middle of an attack.
const JUMP_ATTACK_NOT_END: SwitchSequenceOnLand =
    SwitchSequenceOnLand(CharacterSequenceId::DashDescendLand);

/// Switch to the appropriate sequence based on velocity after attacking.
const JUMP_ATTACK_END: SwitchSequenceOnEndYVelocity = SwitchSequenceOnEndYVelocity {
    upwards: CharacterSequenceId::JumpAscend,
    downwards: CharacterSequenceId::JumpDescend,
};

#[derive(Debug)]
pub(crate) struct JumpAttack;

impl CharacterSequenceHandler for JumpAttack {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        JUMP_ATTACK_NOT_END
            .update(components)
            .or_else(|| JUMP_ATTACK_END.update(components))
    }
}

#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceId, play::RunCounter};
    use game_input::ControllerInput;
    use object_model::play::{Grounding, HealthPoints, Mirrored, Position, Velocity};
    use sequence_model::play::SequenceStatus;

    use super::JumpAttack;
    use crate::sequence_handler::{CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn no_update_when_sequence_not_ended_and_not_on_ground() {
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            None,
            JumpAttack::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceId::JumpAttack,
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
            Some(CharacterSequenceId::DashDescendLand),
            JumpAttack::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceId::FallForwardDescend,
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
            Some(CharacterSequenceId::JumpAscend),
            JumpAttack::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceId::JumpAttack,
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
            Some(CharacterSequenceId::JumpDescend),
            JumpAttack::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceId::JumpAttack,
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
