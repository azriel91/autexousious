/// States used to track X axis input over time to determine when a character should run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunCounter {
    /// Used when there has not been any X axis input for the number of ticks defined by
    /// [`RESET_TICK_COUNT`](#enum.const.RESET_TICK_COUNT).
    Unused,
    /// Used when there is input on the X axis in the positive direction.
    RightIncrease(u32),
    /// Used when input on the X axis in the positive direction has exceeded. [`RESET_TICK_COUNT`]
    /// (#enum.const.RESET_TICK_COUNT) ticks.
    RightExceeded,
    /// Used when there is no input on the X axis, where previously there was in the positive
    /// direction, which was released within (#enum.const.RESET_TICK_COUNT) ticks.
    RightDecrease(u32),
    /// Used when there is input on the X axis in the negative direction.
    LeftIncrease(u32),
    /// Used when input on the X axis in the negative direction has exceeded. [`RESET_TICK_COUNT`]
    /// (#enum.const.RESET_TICK_COUNT) ticks.
    LeftExceeded,
    /// Used when there is no input on the X axis, where previously there was in the negative
    /// direction, which was released within (#enum.const.RESET_TICK_COUNT) ticks.
    LeftDecrease(u32),
}

impl RunCounter {
    /// Number of ticks that the run counter will wait for X axis input to be released / re-pressed
    /// to cause the character to run.
    pub const RESET_TICK_COUNT: u32 = 15;
}

impl Default for RunCounter {
    fn default() -> Self {
        RunCounter::Unused
    }
}
