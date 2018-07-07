use amethyst::prelude::*;

use GameUpdate;

/// Runs an effect function in `.update()` then switches to the next state.
#[derive(Debug, new)]
pub struct EffectState<F>
where
    F: Fn(&mut World),
{
    /// Function that asserts the expected program state.
    effect_fn: F,
}

impl<F, T> State<T> for EffectState<F>
where
    F: Fn(&mut World),
    T: GameUpdate,
{
    fn update(&mut self, mut data: StateData<T>) -> Trans<T> {
        data.data.update(&data.world);

        (self.effect_fn)(&mut data.world);

        Trans::Pop
    }
}
