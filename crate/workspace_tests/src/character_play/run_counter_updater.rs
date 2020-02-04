#[cfg(test)]
mod tests {
    use character_model::{config::CharacterSequenceName, play::RunCounter};
    use game_input_model::play::ControllerInput;
    use mirrored_model::play::Mirrored;
    use object_model::play::Grounding;
    use sequence_model::config::SequenceNameString;

    use character_play::RunCounterUpdater;

    #[test]
    fn unused_when_not_stand_or_walk() {
        let input = ControllerInput::default();

        assert_eq!(
            RunCounter::Unused,
            RunCounterUpdater::update(
                RunCounter::Unused,
                &input,
                &SequenceNameString::Name(CharacterSequenceName::Jump),
                Mirrored::default(),
                Grounding::Airborne
            )
        );
    }

    #[test]
    fn none_when_grounding_is_airborne_and_unused() {
        let input = ControllerInput::default();

        assert_eq!(
            RunCounter::Unused,
            RunCounterUpdater::update(
                RunCounter::Unused,
                &input,
                &SequenceNameString::Name(CharacterSequenceName::default()),
                Mirrored::default(),
                Grounding::Airborne
            )
        );
    }

    #[test]
    fn unused_when_grounding_is_airborne() {
        let input = ControllerInput::default();

        assert_eq!(
            RunCounter::Unused,
            RunCounterUpdater::update(
                RunCounter::Increase(10),
                &input,
                &SequenceNameString::Name(CharacterSequenceName::default()),
                Mirrored::default(),
                Grounding::Airborne
            )
        );
    }

    #[test]
    fn none_when_unused_and_no_x_input() {
        let input = ControllerInput::default();

        assert_eq!(
            RunCounter::Unused,
            RunCounterUpdater::update(
                RunCounter::Unused,
                &input,
                &SequenceNameString::Name(CharacterSequenceName::default()),
                Mirrored::default(),
                Grounding::OnGround
            )
        );
    }

    #[test]
    fn unused_when_counter_decrease_runs_out_and_no_x_input() {
        let input = ControllerInput::default();

        assert_eq!(
            RunCounter::Unused,
            RunCounterUpdater::update(
                RunCounter::Decrease(0),
                &input,
                &SequenceNameString::Name(CharacterSequenceName::default()),
                Mirrored::default(),
                Grounding::OnGround
            )
        );
    }

    #[test]
    fn decrements_run_counter_when_decrease_and_no_x_input() {
        let input = ControllerInput::default();

        assert_eq!(
            RunCounter::Decrease(0),
            RunCounterUpdater::update(
                RunCounter::Decrease(1),
                &input,
                &SequenceNameString::Name(CharacterSequenceName::default()),
                Mirrored::default(),
                Grounding::OnGround
            )
        );
    }

    #[test]
    fn decrease_when_increase_and_no_x_input() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            RunCounter::Decrease(RunCounter::RESET_TICK_COUNT),
            RunCounterUpdater::update(
                RunCounter::Increase(0),
                &input,
                &SequenceNameString::Name(CharacterSequenceName::default()),
                Mirrored::default(),
                Grounding::OnGround
            )
        );
    }

    #[test]
    fn increase_when_unused_and_input_non_zero() {
        let x_inputs = vec![1., -1.];
        let mirrors = vec![false, true];

        x_inputs
            .into_iter()
            .zip(mirrors.into_iter())
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Increase(RunCounter::RESET_TICK_COUNT),
                    RunCounterUpdater::update(
                        RunCounter::Unused,
                        &input,
                        &SequenceNameString::Name(CharacterSequenceName::default()),
                        mirrored.into(),
                        Grounding::OnGround
                    )
                );
            });
    }

    #[test]
    fn increase_when_decrease_input_different_direction() {
        vec![(1., true), (-1., false)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Increase(RunCounter::RESET_TICK_COUNT),
                    RunCounterUpdater::update(
                        RunCounter::Decrease(11),
                        &input,
                        &SequenceNameString::Name(CharacterSequenceName::default()),
                        mirrored.into(),
                        Grounding::OnGround
                    )
                );
            });
    }

    #[test]
    fn increase_when_increase_input_different_direction() {
        vec![(1., true), (-1., false)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Increase(RunCounter::RESET_TICK_COUNT),
                    RunCounterUpdater::update(
                        RunCounter::Increase(11),
                        &input,
                        &SequenceNameString::Name(CharacterSequenceName::default()),
                        mirrored.into(),
                        Grounding::OnGround
                    )
                );
            });
    }

    #[test]
    fn unused_when_decrease_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Unused,
                    RunCounterUpdater::update(
                        RunCounter::Decrease(11),
                        &input,
                        &SequenceNameString::Name(CharacterSequenceName::default()),
                        mirrored.into(),
                        Grounding::OnGround
                    )
                );
            });
    }

    #[test]
    fn exceeded_when_input_positive_same_direction_and_exceeds_tick_count() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Exceeded,
                    RunCounterUpdater::update(
                        RunCounter::Increase(0),
                        &input,
                        &SequenceNameString::Name(CharacterSequenceName::default()),
                        mirrored.into(),
                        Grounding::OnGround
                    )
                );
            });
    }

    #[test]
    fn decrements_increase_value_when_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Increase(10),
                    RunCounterUpdater::update(
                        RunCounter::Increase(11),
                        &input,
                        &SequenceNameString::Name(CharacterSequenceName::default()),
                        mirrored.into(),
                        Grounding::OnGround
                    )
                );
            });
    }

    #[test]
    fn none_when_exceeded_and_input_same_direction() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = ControllerInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    RunCounter::Exceeded,
                    RunCounterUpdater::update(
                        RunCounter::Exceeded,
                        &input,
                        &SequenceNameString::Name(CharacterSequenceName::default()),
                        mirrored.into(),
                        Grounding::OnGround
                    )
                );
            });
    }
}
