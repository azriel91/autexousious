#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input_model::play::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use mirrored_model::play::Mirrored;
    use object_model::play::{Grounding, HealthPoints};
    use sequence_model::play::SequenceStatus;

    use character_play::{
        sequence_handler::{CharacterSequenceHandler, Walk},
        CharacterSequenceUpdateComponents,
    };

    #[test]
    fn walk_when_x_axis_positive_mirror() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::OnGround,
                RunCounter::Increase(11)
            ))
        );
    }

    #[test]
    fn walk_when_x_axis_negative_non_mirror() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::OnGround,
                RunCounter::Increase(11)
            ))
        );
    }

    #[test]
    fn walk_when_z_axis_non_zero() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            None,
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::OnGround,
                RunCounter::Increase(0)
            ))
        );

        let input = ControllerInput::new(0., -1., false, false, false, false);

        assert_eq!(
            None,
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::OnGround,
                RunCounter::Increase(0)
            ))
        );
    }

    #[test]
    fn restarts_walk_when_sequence_ended() {
        vec![(0., 1.), (0., -1.)]
            .into_iter()
            .for_each(|(x_input, z_input)| {
                let input = ControllerInput::new(x_input, z_input, false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceName::Walk),
                    Walk::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        CharacterSequenceName::Walk,
                        SequenceStatus::End,
                        &Position::default(),
                        &Velocity::default(),
                        Mirrored(false),
                        Grounding::OnGround,
                        RunCounter::Increase(0)
                    ))
                );
            });

        vec![(1., 1., false), (-1., -1., true)]
            .into_iter()
            .for_each(|(x_input, z_input, mirrored)| {
                let input = ControllerInput::new(x_input, z_input, false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceName::Walk),
                    Walk::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        CharacterSequenceName::Walk,
                        SequenceStatus::End,
                        &Position::default(),
                        &Velocity::default(),
                        mirrored.into(),
                        Grounding::OnGround,
                        RunCounter::Increase(1)
                    ))
                );
            });
    }

    #[test]
    fn run_when_x_axis_positive_and_run_counter_decrease_non_mirror() {
        let input = ControllerInput::new(1., -1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Run),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::OnGround,
                RunCounter::Decrease(10)
            ))
        );
    }

    #[test]
    fn run_when_x_axis_negative_and_run_counter_decrease_mirror() {
        let input = ControllerInput::new(-1., -1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Run),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::OnGround,
                RunCounter::Decrease(10)
            ))
        );
    }
}
