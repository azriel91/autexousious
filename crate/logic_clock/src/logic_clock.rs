use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_new::new;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Logical clock that has a value and limit.
#[derive(
    Clone, Component, Copy, Debug, Default, Deserialize, Hash, PartialEq, Eq, Serialize, new,
)]
pub struct LogicClock {
    /// Current value of this clock.
    #[new(default)]
    pub value: usize,
    /// Limit of this clock.
    pub limit: usize,
}

impl LogicClock {
    /// Returns a new `LogicClock` with an initial value.
    pub fn new_with_value(limit: usize, value: usize) -> Self {
        Self { value, limit }
    }

    /// Returns whether this clock has reached its limit.
    pub fn is_beginning(self) -> bool {
        self.value == 0
    }

    /// Returns whether this clock has reached its limit.
    pub fn is_complete(self) -> bool {
        self.value == self.limit
    }

    /// Returns whether this clock has ticked, but not yet reached its limit.
    pub fn is_ongoing(self) -> bool {
        // Logically the same as:
        //
        // ```rust
        // !self.is_beginning() && !self.is_complete()
        // ```
        //
        // but less operations.
        !(self.is_beginning() || self.is_complete())
    }

    /// Increments this clock's value if it hasn't reached its limit.
    ///
    /// **Note:**
    ///
    /// This will not increment the value past the limit, nor will it reset or wrap the value. You
    /// should use the [LogicClock::reset] method to do this.
    pub fn tick(&mut self) {
        if self.value < self.limit {
            self.value += 1;
        }
    }

    /// Decrements this clock's value.
    ///
    /// **Note:**
    ///
    /// This will not decrement the value past 0, nor will it wrap the value. You can use the
    /// [LogicClock::complete] method to do this.
    pub fn reverse_tick(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }

    /// Resets this clock's value back to 0.
    pub fn reset(&mut self) {
        self.value = 0;
    }

    /// Sets this clocks value to its limit.
    pub fn complete(&mut self) {
        self.value = self.limit;
    }
}

#[cfg(test)]
mod tests {
    use super::LogicClock;

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
