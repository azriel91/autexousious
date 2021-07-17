use amethyst::ecs::{storage::DenseVecStorage, Component};
use derivative::Derivative;

/// States used to track X axis input over time to determine when a character
/// should run.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum RunCounter {
    /// Used when there has not been any X axis input for the number of ticks
    /// defined by [`RESET_TICK_COUNT`](#enum.const.RESET_TICK_COUNT).
    #[derivative(Default)]
    Unused,
    /// Used while the character is walking and there is input on the X axis.
    Increase(u32),
    /// Used when input on the X axis in the same direction has exceeded.
    /// [`RESET_TICK_COUNT`] (#enum.const.RESET_TICK_COUNT) ticks.
    Exceeded,
    /// Used in the `Stand`ing state, while there is no input on the X axis,
    /// where previously the character was `Walk`ing and reverted to
    /// `Stand`ing within (#enum.const.RESET_TICK_COUNT) ticks.
    Decrease(u32), // kcov-ignore
}

impl RunCounter {
    /// Number of ticks that the run counter will wait for X axis input to be
    /// released / re-pressed to cause the character to run.
    pub const RESET_TICK_COUNT: u32 = 15;
}

impl Component for RunCounter {
    type Storage = DenseVecStorage<Self>;
}
