#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use character_play::{
        sequence_handler::{CharacterSequenceHandler, JumpAttack},
        CharacterSequenceUpdateComponents,
    };

    #[test]
    fn no_update_when_sequence_not_ended_and_not_on_ground() {
        let mut velocity = Velocity::default();
        velocity[1] = -1.;

        assert_eq!(
            None,
            JumpAttack::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::JumpAttack,
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
            JumpAttack::update(CharacterSequenceUpdateComponents::new(
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
            Some(CharacterSequenceName::JumpAscend),
            JumpAttack::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::JumpAttack,
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
            Some(CharacterSequenceName::JumpDescend),
            JumpAttack::update(CharacterSequenceUpdateComponents::new(
                &ControllerInput::default(),
                HealthPoints::default(),
                CharacterSequenceName::JumpAttack,
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
