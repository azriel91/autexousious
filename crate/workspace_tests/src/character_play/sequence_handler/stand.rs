#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input_model::play::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use mirrored_model::play::Mirrored;
    use object_model::play::{Grounding, HealthPoints};
    use sequence_model::play::SequenceStatus;

    use character_play::{
        sequence_handler::{CharacterSequenceHandler, Stand},
        CharacterSequenceUpdateComponents,
    };

    #[test]
    fn no_change_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            None,
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::OnGround,
                RunCounter::Unused
            ))
        );
    }

    #[test]
    fn restarts_stand_when_no_input_and_sequence_end() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Stand),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Stand,
                SequenceStatus::End,
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn switches_to_jump_descend_when_airborne() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::JumpDescend),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::Airborne,
                RunCounter::default()
            ))
        );
    }

    #[test]
    #[should_panic(expected = "Invalid run_counter state")]
    fn panics_when_run_counter_exceeded() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        Stand::update(CharacterSequenceUpdateComponents::new(
            &input,
            HealthPoints::default(),
            CharacterSequenceName::default(),
            SequenceStatus::default(),
            &Position::default(),
            &Velocity::default(),
            Mirrored::default(),
            Grounding::OnGround,
            RunCounter::Exceeded,
        ));
    } // kcov-ignore

    #[test]
    #[should_panic(expected = "Invalid run_counter state")]
    fn panics_when_run_counter_increase() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        Stand::update(CharacterSequenceUpdateComponents::new(
            &input,
            HealthPoints::default(),
            CharacterSequenceName::default(),
            SequenceStatus::default(),
            &Position::default(),
            &Velocity::default(),
            Mirrored::default(),
            Grounding::OnGround,
            RunCounter::Increase(10),
        ));
    } // kcov-ignore

    #[test]
    fn walk_when_x_axis_is_positive_mirrored() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );

        // Already facing right
        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn walk_when_x_axis_is_negative_non_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );

        // Already facing left
        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn walk_when_x_and_z_axes_are_non_zero() {
        let input = ControllerInput::new(1., 1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceName::Walk),
            Stand::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceName::Stand,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::OnGround,
                RunCounter::default()
            ))
        );
    }

    #[test]
    fn run_when_run_counter_decrease_x_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceName::Run),
                    Stand::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        CharacterSequenceName::Stand,
                        SequenceStatus::default(),
                        &Position::default(),
                        &Velocity::default(),
                        mirrored.into(),
                        Grounding::OnGround,
                        RunCounter::Decrease(10)
                    ))
                );
            });
    }

    #[test]
    fn walk_when_run_counter_decrease_x_input_different_direction() {
        vec![(1., true), (-1., false)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceName::Walk),
                    Stand::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        CharacterSequenceName::Stand,
                        SequenceStatus::default(),
                        &Position::default(),
                        &Velocity::default(),
                        mirrored.into(),
                        Grounding::OnGround,
                        RunCounter::Decrease(10)
                    ))
                );
            });
    }
}
