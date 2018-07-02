use std::marker::PhantomData;

use amethyst::prelude::*;

use EmptyState;
use GameUpdate;

/// Runs an effect function in `.update()` then switches to the next state.
#[derive(Debug)]
pub struct EffectState<F, SStack, SNext, T>
where
    F: Fn(&mut World),
    SStack: State<T>,
    SNext: State<T>,
{
    /// Function that asserts the expected program state.
    effect_fn: F,
    /// `State` to stack over this one before running the effect.
    stack_state: Option<SStack>,
    /// `State` to switch to after this one has been run.
    next_state: Option<SNext>,
    /// Marker for game data.
    game_data: PhantomData<T>,
}

impl<F, SNext, T> EffectState<F, EmptyState, SNext, T>
where
    F: Fn(&mut World),
    SNext: State<T>,
{
    /// Returns a new `EffectState` with the given assertion function.
    ///
    /// # Parameters:
    ///
    /// * `effect_fn`: Function that asserts the expected program state.
    /// * `state`: `State` to switch to after this one has been run.
    pub fn new(effect_fn: F, state: SNext) -> Self {
        EffectState {
            effect_fn,
            stack_state: None,
            next_state: Some(state),
            game_data: PhantomData,
        }
    }
}

impl<F, SStack, SNext, T> EffectState<F, SStack, SNext, T>
where
    F: Fn(&mut World),
    SStack: State<T>,
    SNext: State<T>,
{
    /// Registers a stack state to this effect_state.
    ///
    /// # Parameters
    ///
    /// * `state`: `State` that should run before this one.
    pub fn with_stack_state<SStackLocal>(
        self,
        state: SStackLocal,
    ) -> EffectState<F, SStackLocal, SNext, T>
    where
        SStackLocal: State<T>,
    {
        EffectState {
            effect_fn: self.effect_fn,
            stack_state: Some(state),
            next_state: self.next_state,
            game_data: PhantomData,
        }
    }
}

impl<F, SStack, SNext, T> State<T> for EffectState<F, SStack, SNext, T>
where
    F: Fn(&mut World),
    SStack: State<T> + 'static,
    SNext: State<T> + 'static,
    T: GameUpdate,
{
    fn update(&mut self, mut data: StateData<T>) -> Trans<T> {
        data.data.update(&data.world);

        if self.stack_state.is_some() {
            return Trans::Push(Box::new(self.stack_state.take().unwrap()));
        }

        (self.effect_fn)(&mut data.world);

        Trans::Switch(Box::new(
            self.next_state
                .take()
                .expect("Expected next state to be set."),
        ))
    }
}
