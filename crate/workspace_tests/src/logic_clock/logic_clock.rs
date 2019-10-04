#[cfg(test)]
mod tests {
    use logic_clock::LogicClock;

    #[test]
    fn is_beginning_when_value_is_0() {
        let logic_clock = LogicClock::new(3);
        assert!(logic_clock.is_beginning());
    }

    #[test]
    fn is_complete_when_value_equals_limit() {
        let mut logic_clock = LogicClock::new(3);
        assert!(!logic_clock.is_complete());

        logic_clock.value = 1;
        assert!(!logic_clock.is_complete());
        logic_clock.value = 2;
        assert!(!logic_clock.is_complete());

        logic_clock.value = 3;
        assert!(logic_clock.is_complete());
    }

    #[test]
    fn is_ongoing_when_value_is_between_0_and_limit() {
        let mut logic_clock = LogicClock::new(3);
        assert!(!logic_clock.is_ongoing());

        logic_clock.value = 1;
        assert!(logic_clock.is_ongoing());
        logic_clock.value = 2;
        assert!(logic_clock.is_ongoing());

        logic_clock.value = 3;
        assert!(!logic_clock.is_ongoing());
    }

    #[test]
    fn tick_increments_value_by_one() {
        let mut logic_clock = LogicClock::new(3);
        assert_eq!(0, logic_clock.value);

        logic_clock.tick();
        assert_eq!(1, logic_clock.value);
        logic_clock.tick();
        assert_eq!(2, logic_clock.value);

        logic_clock.tick();
        assert_eq!(3, logic_clock.value);
    }

    #[test]
    fn tick_does_not_go_past_limit_or_wrap_value() {
        let mut logic_clock = LogicClock::new_with_value(3, 3);

        logic_clock.tick();
        assert_eq!(3, logic_clock.value);
        logic_clock.tick();
        assert_eq!(3, logic_clock.value);
    }

    #[test]
    fn reset_sets_value_to_0() {
        let mut logic_clock = LogicClock::new(3);
        assert_eq!(0, logic_clock.value);

        logic_clock.tick();
        assert_eq!(1, logic_clock.value);

        logic_clock.reset();
        assert_eq!(0, logic_clock.value);
    }

    #[test]
    fn reverse_tick_decrements_value_by_one() {
        let mut logic_clock = LogicClock::new_with_value(3, 3);
        assert_eq!(3, logic_clock.value);

        logic_clock.reverse_tick();
        assert_eq!(2, logic_clock.value);
        logic_clock.reverse_tick();
        assert_eq!(1, logic_clock.value);

        logic_clock.reverse_tick();
        assert_eq!(0, logic_clock.value);
    }

    #[test]
    fn reverse_tick_does_not_go_past_zero_or_wrap_value() {
        let mut logic_clock = LogicClock::new(3);

        logic_clock.reverse_tick();
        assert_eq!(0, logic_clock.value);
        logic_clock.reverse_tick();
        assert_eq!(0, logic_clock.value);
    }

    #[test]
    fn complete_sets_value_to_limit() {
        let mut logic_clock = LogicClock::new(3);
        assert_eq!(0, logic_clock.value);

        logic_clock.tick();
        assert_eq!(1, logic_clock.value);

        logic_clock.complete();
        assert_eq!(3, logic_clock.value);
    }
}
