#[cfg(test)]
mod tests {
    use game_input_model::config::InputDirection;

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
