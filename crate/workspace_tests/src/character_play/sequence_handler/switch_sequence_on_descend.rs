#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use mirrored_model::play::Mirrored;
    use object_model::play::{Grounding, HealthPoints};
    use sequence_model::play::SequenceStatus;

    use character_play::{
        sequence_handler::SwitchSequenceOnDescend, CharacterSequenceUpdateComponents,
    };

    #[test]
    fn no_update_when_sequence_not_ended() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut velocity = Velocity::default();
        velocity[1] = 1.;

        assert_eq!(
            None,
            SwitchSequenceOnDescend(CharacterSequenceName::FallForwardDescend).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    HealthPoints::default(),
                    CharacterSequenceName::FallForwardAscend,
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
    fn restarts_ascend_sequence_when_sequence_ends() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut velocity = Velocity::default();
        velocity[1] = 1.;

        assert_eq!(
            Some(CharacterSequenceName::FallForwardAscend),
            SwitchSequenceOnDescend(CharacterSequenceName::FallForwardDescend).update(
                CharacterSequenceUpdateComponents::new(
                    &input,
                    HealthPoints::default(),
                    CharacterSequenceName::FallForwardAscend,
                    SequenceStatus::End,
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
    fn switches_to_descend_sequence_when_y_velocity_is_zero_or_downwards() {
        let input = ControllerInput::new(0., 0., false, false, false, false);
        let mut downwards_velocity = Velocity::default();
        downwards_velocity[1] = -1.;

        vec![Velocity::default(), downwards_velocity]
            .into_iter()
            .for_each(|velocity| {
                assert_eq!(
                    Some(CharacterSequenceName::FallForwardDescend),
                    SwitchSequenceOnDescend(CharacterSequenceName::FallForwardDescend).update(
                        CharacterSequenceUpdateComponents::new(
                            &input,
                            HealthPoints::default(),
                            CharacterSequenceName::FallForwardAscend,
                            SequenceStatus::Ongoing,
                            &Position::default(),
                            &velocity,
                            Mirrored::default(),
                            Grounding::Airborne,
                            RunCounter::default()
                        )
                    )
                );
            });
    }
}
