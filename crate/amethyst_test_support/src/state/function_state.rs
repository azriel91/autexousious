use amethyst::prelude::*;

use GameUpdate;

/// Runs a function in `.update()` then `Pop`s itself.
#[derive(Debug, new)]
pub struct FunctionState<F>
where
    F: Fn(&mut World),
{
    /// Function to run in `update`.
    function: F,
}

impl<F, T> State<T> for FunctionState<F>
where
    F: Fn(&mut World),
    T: GameUpdate,
{
    fn update(&mut self, mut data: StateData<T>) -> Trans<T> {
        data.data.update(&data.world);

        (self.function)(&mut data.world);

        Trans::Pop
    }
}
