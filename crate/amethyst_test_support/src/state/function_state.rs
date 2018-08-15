use amethyst::prelude::*;

use GameUpdate;

/// Runs a function in `.update()` then `Pop`s itself.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct FunctionState {
    /// Function to run in `update`.
    #[derivative(Debug = "ignore")]
    function: Box<Fn(&mut World)>,
}

impl<T> State<T> for FunctionState
where
    T: GameUpdate,
{
    fn update(&mut self, mut data: StateData<T>) -> Trans<T> {
        data.data.update(&data.world);

        (self.function)(&mut data.world);

        Trans::Pop
    }
}
