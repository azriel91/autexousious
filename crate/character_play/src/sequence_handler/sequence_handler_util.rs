use game_input::ControllerInput;
use object_model::play::Mirrored;

#[derive(Debug)]
pub struct SequenceHandlerUtil;

impl SequenceHandlerUtil {
    /// Returns whether X axis input is in the same direction as the object is facing.
    ///
    /// This returns `false` if there is no input on the X axis.
    ///
    /// # Parameters
    ///
    /// * `controller_input`: Controller input for the character.
    /// * `mirrored`: Whether the character is facing left.
    pub fn input_matches_direction(controller_input: &ControllerInput, mirrored: Mirrored) -> bool {
        controller_input.x_axis_value > 0. && !mirrored.0
            || controller_input.x_axis_value < 0. && mirrored.0
    }

    /// Returns whether X axis input is in the opposite direction as the object is facing.
    ///
    /// This returns `false` if there is no input on the X axis.
    ///
    /// # Parameters
    ///
    /// * `controller_input`: Controller input for the character.
    /// * `mirrored`: Whether the character is facing left.
    pub fn input_opposes_direction(controller_input: &ControllerInput, mirrored: Mirrored) -> bool {
        controller_input.x_axis_value > 0. && mirrored.0
            || controller_input.x_axis_value < 0. && !mirrored.0
    }
}

#[cfg(test)]
mod tests {
    use game_input::ControllerInput;
    use object_model::play::Mirrored;

    use super::SequenceHandlerUtil;

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
