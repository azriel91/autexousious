#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use mirrored_model::play::Mirrored;

    use character_play::sequence_handler::SequenceHandlerUtil;

    #[test]
    fn matches_direction_returns_false_on_no_x_input() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            false,
            SequenceHandlerUtil::input_matches_direction(&input, Mirrored(true))
        );
        assert_eq!(
            false,
            SequenceHandlerUtil::input_matches_direction(&input, Mirrored(false))
        );
    }

    #[test]
    fn matches_direction_returns_false_on_positive_x_input_and_mirrored() {
        let input = ControllerInput::new(1., 1., false, false, false, false);

        assert_eq!(
            false,
            SequenceHandlerUtil::input_matches_direction(&input, Mirrored(true))
        );
    }

    #[test]
    fn matches_direction_returns_false_on_negative_x_input_and_not_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            false,
            SequenceHandlerUtil::input_matches_direction(&input, Mirrored(false))
        );
    }

    #[test]
    fn matches_direction_returns_true_on_positive_x_input_and_not_mirrored() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert!(SequenceHandlerUtil::input_matches_direction(
            &input,
            Mirrored(false)
        ));
    }

    #[test]
    fn matches_direction_returns_true_on_negative_x_input_and_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert!(SequenceHandlerUtil::input_matches_direction(
            &input,
            Mirrored(true)
        ));
    }

    #[test]
    fn opposes_direction_returns_false_on_no_x_input() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            false,
            SequenceHandlerUtil::input_opposes_direction(&input, Mirrored(true))
        );
        assert_eq!(
            false,
            SequenceHandlerUtil::input_opposes_direction(&input, Mirrored(false))
        );
    }

    #[test]
    fn opposes_direction_returns_false_on_positive_x_input_and_not_mirrored() {
        let input = ControllerInput::new(1., 1., false, false, false, false);

        assert_eq!(
            false,
            SequenceHandlerUtil::input_opposes_direction(&input, Mirrored(false))
        );
    }

    #[test]
    fn opposes_direction_returns_false_on_negative_x_input_and_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            false,
            SequenceHandlerUtil::input_opposes_direction(&input, Mirrored(true))
        );
    }

    #[test]
    fn opposes_direction_returns_true_on_positive_x_input_and_mirrored() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert!(SequenceHandlerUtil::input_opposes_direction(
            &input,
            Mirrored(true)
        ));
    }

    #[test]
    fn opposes_direction_returns_true_on_negative_x_input_and_not_mirrored() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert!(SequenceHandlerUtil::input_opposes_direction(
            &input,
            Mirrored(false)
        ));
    }
}
