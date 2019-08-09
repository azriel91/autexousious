use serde::{Deserialize, Serialize};

/// Variants for axis input matching.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum InputDirection {
    /// Axis input is zero.
    None,
    /// Axis input is in the same direction to the character is facing.
    Same,
    /// Axis input is opposite to the direction the character is facing.
    Mirrored,
    /// Axis input is non-zero.
    Some,
    /// Axis input is zero or opposite to the direction the character is facing.
    NotSame,
    /// Axis input is zero or in the same direction to the character is facing.
    NotMirrored,
}

impl InputDirection {
    /// Returns whether axis input is in the same direction as the object is facing.
    ///
    /// This returns `false` if there is no input on the axis.
    ///
    /// # Parameters
    ///
    /// * `axis_value`: Input value of the axis.
    /// * `mirrored`: Whether the object is facing the axis negative direction.
    pub fn input_matches_direction(axis_value: f32, mirrored: bool) -> bool {
        axis_value > 0. && !mirrored || axis_value < 0. && mirrored
    }

    /// Returns whether axis input is in the opposite direction as the object is facing.
    ///
    /// This returns `false` if there is no input on the axis.
    ///
    /// # Parameters
    ///
    /// * `axis_value`: Input value of the axis.
    /// * `mirrored`: Whether the object is facing the axis negative direction.
    pub fn input_opposes_direction(axis_value: f32, mirrored: bool) -> bool {
        axis_value > 0. && mirrored || axis_value < 0. && !mirrored
    }
}

#[cfg(test)]
mod tests {
    use super::InputDirection;

    #[test]
    fn matches_direction_returns_false_on_no_input() {
        assert!(!InputDirection::input_matches_direction(0., true));
        assert!(!InputDirection::input_matches_direction(0., false));
    }

    #[test]
    fn matches_direction_returns_false_on_positive_input_and_mirrored() {
        assert!(!InputDirection::input_matches_direction(1., true));
    }

    #[test]
    fn matches_direction_returns_false_on_negative_input_and_not_mirrored() {
        assert!(!InputDirection::input_matches_direction(-1., false));
    }

    #[test]
    fn matches_direction_returns_true_on_positive_input_and_not_mirrored() {
        assert!(InputDirection::input_matches_direction(1., false));
    }

    #[test]
    fn matches_direction_returns_true_on_negative_input_and_mirrored() {
        assert!(InputDirection::input_matches_direction(-1., true));
    }

    #[test]
    fn opposes_direction_returns_false_on_no_input() {
        assert!(!InputDirection::input_opposes_direction(0., true));
        assert!(!InputDirection::input_opposes_direction(0., false));
    }

    #[test]
    fn opposes_direction_returns_false_on_positive_input_and_not_mirrored() {
        assert!(!InputDirection::input_opposes_direction(1., false));
    }

    #[test]
    fn opposes_direction_returns_false_on_negative_input_and_mirrored() {
        assert!(!InputDirection::input_opposes_direction(-1., true));
    }

    #[test]
    fn opposes_direction_returns_true_on_positive_input_and_mirrored() {
        assert!(InputDirection::input_opposes_direction(1., true));
    }

    #[test]
    fn opposes_direction_returns_true_on_negative_input_and_not_mirrored() {
        assert!(InputDirection::input_opposes_direction(-1., false));
    }
}
