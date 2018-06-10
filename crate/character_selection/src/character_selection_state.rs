use std::fmt::Debug;
use std::marker::PhantomData;

use amethyst::prelude::*;

use CharacterSelection;

/// `State` where character selection takes place.
///
/// # Type Parameters
///
/// * `F`: Function to construct the state to return after character selection is complete.
/// * `S`: State to return.
/// * `T`: Data type used by this state and the returned state (see `StateData`).
#[derive(Derivative)]
#[derivative(Debug)]
pub struct CharacterSelectionState<F, S, T>
where
    F: Fn() -> Box<S>,
    S: State<T> + 'static,
{
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "F: Debug"))]
    next_state_fn: Box<F>,
    /// Data type used by this state and the returned state (see `StateData`).
    state_data: PhantomData<T>,
}

impl<F, S, T> CharacterSelectionState<F, S, T>
where
    F: Fn() -> Box<S>,
    S: State<T> + 'static,
{
    /// Returns a new `CharacterSelectionState`
    ///
    /// # Parameters
    ///
    /// * `next_state_fn`: Function that returns the `State` to transition to when characters have
    ///     been selected
    pub fn new(next_state_fn: Box<F>) -> Self {
        CharacterSelectionState {
            next_state_fn,
            state_data: PhantomData,
        }
    }
}

impl<'a, 'b, F, S, T> State<T> for CharacterSelectionState<F, S, T>
where
    F: Fn() -> Box<S>,
    S: State<T> + 'static,
{
    fn fixed_update(&mut self, data: StateData<T>) -> Trans<T> {
        let selected_characters = data.world.read_resource::<CharacterSelection>();
        if selected_characters.is_empty() {
            Trans::None
        } else {
            info!("selected_characters: `{:?}`", &*selected_characters);

            // TODO: `Trans:Push` when we have a proper character selection menu.
            Trans::Switch((self.next_state_fn)())
        }
    }
}
