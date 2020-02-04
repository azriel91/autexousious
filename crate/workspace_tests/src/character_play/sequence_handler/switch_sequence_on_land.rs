#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input_model::play::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use mirrored_model::play::Mirrored;
    use object_model::play::{Grounding, HealthPoints};
    use sequence_model::play::SequenceStatus;

    use character_play::{
        sequence_handler::SwitchSequenceOnLand, CharacterSequenceUpdateComponents,
    };

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
                    CharacterSequenceName::FallForwardDescend,
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
                    CharacterSequenceName::FallForwardDescend,
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
