use std::marker::PhantomData;

use amethyst::prelude::*;

use GameUpdate;

/// Runs an effect function in `.update()` then switches to the next state.
#[derive(Debug)]
pub struct EffectState<F, S, T>
where
    F: Fn(&mut World),
    S: State<T>,
{
    /// Function that asserts the expected program state.
    effect_fn: F,
    /// `State` to switch to after this one has been run.
    next_state: Option<S>,
    /// Marker for game data.
    game_data: PhantomData<T>,
}

impl<F, S, T> EffectState<F, S, T>
where
    F: Fn(&mut World),
    S: State<T>,
{
    /// Returns a new `EffectState` with the given assertion function.
    ///
    /// # Parameters:
    ///
    /// * `effect_fn`: Function that asserts the expected program state.
    /// * `state`: `State` to switch to after this one has been run.
    pub fn new(effect_fn: F, state: S) -> Self {
        EffectState {
            effect_fn,
            next_state: Some(state),
            game_data: PhantomData,
        }
    }
}

impl<F, S, T> State<T> for EffectState<F, S, T>
where
    F: Fn(&mut World),
    S: State<T> + 'static,
    T: GameUpdate,
{
    fn update(&mut self, mut data: StateData<T>) -> Trans<T> {
        data.data.update(&data.world);
        (self.effect_fn)(&mut data.world);

        Trans::Switch(Box::new(
            self.next_state
                .take()
                .expect("Expected next state to be set."),
        ))
    }
}
