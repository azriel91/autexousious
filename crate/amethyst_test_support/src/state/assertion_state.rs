use amethyst::prelude::*;

/// Runs an assertion function in `.update()` then returns `Trans::Pop`.
#[derive(Debug, new)]
pub struct AssertionState<F>
where
    F: Fn(&mut World),
{
    assertion_fn: F,
}

impl<'a, 'b, F> State<GameData<'a, 'b>> for AssertionState<F>
where
    F: Fn(&mut World),
{
    fn update(&mut self, mut data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);

        (self.assertion_fn)(&mut data.world);

        Trans::Pop
    }
}
