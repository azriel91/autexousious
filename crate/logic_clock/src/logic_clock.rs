use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_new::new;
use serde::{Deserialize, Serialize};

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
    /// This will not increment the value past the limit, nor will it reset or
    /// wrap the value. You should use the [LogicClock::reset] method to do
    /// this.
    pub fn tick(&mut self) {
        if self.value < self.limit {
            self.value += 1;
        }
    }

    /// Decrements this clock's value.
    ///
    /// **Note:**
    ///
    /// This will not decrement the value past 0, nor will it wrap the value.
    /// You can use the [LogicClock::complete] method to do this.
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
