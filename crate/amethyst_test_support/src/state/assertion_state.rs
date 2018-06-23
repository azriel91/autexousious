use std::marker::PhantomData;

use amethyst::prelude::*;

use EmptyState;

/// Runs an assertion function in `.update()` then returns `Trans::Pop`.
#[derive(Debug)]
pub struct AssertionState<F, S, T>
where
    F: Fn(&mut World),
    S: State<T>,
{
    /// Function that asserts the expected program state.
    assertion_fn: F,
    /// `State` to stack over this one before running the assertion.
    stack_state: Option<S>,
    /// Marker for game data.
    game_data: PhantomData<T>,
}

impl<F> AssertionState<F, EmptyState, GameData<'static, 'static>>
where
    F: Fn(&mut World),
{
    /// Returns a new `AssertionState` with the given assertion function.
    ///
    /// # Parameters:
    ///
    /// * `assertion_fn`: Function that asserts the expected program state.
    pub fn new(assertion_fn: F) -> Self {
        AssertionState {
            assertion_fn,
            stack_state: None,
            game_data: PhantomData,
        }
    }

    /// Registers a stack state to this assertion_state.
    ///
    /// # Parameters
    ///
    /// * `state`: `State` that should run before this one.
    pub fn with_stack_state<SLocal, TLocal>(
        self,
        state: SLocal,
    ) -> AssertionState<F, SLocal, TLocal>
    where
        SLocal: State<TLocal>,
    {
        AssertionState {
            assertion_fn: self.assertion_fn,
            stack_state: Some(state),
            game_data: PhantomData,
        }
    }
}

impl<F, S, T> State<T> for AssertionState<F, S, T>
where
    F: Fn(&mut World),
    S: State<T> + 'static,
{
    fn update(&mut self, mut data: StateData<T>) -> Trans<T> {
        if let Some(stack_state) = self.stack_state.take() {
            return Trans::Push(Box::new(stack_state));
        }

        (self.assertion_fn)(&mut data.world);

        Trans::Pop
    }
}
