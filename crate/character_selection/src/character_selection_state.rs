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
#[derive(Derivative)]
#[derivative(Debug)]
pub struct CharacterSelectionState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>> + 'static,
{
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "F: Debug"))]
    next_state_fn: Box<F>,
    /// Data type used by this state and the returned state (see `StateData`).
    state_data: PhantomData<GameData<'a, 'b>>,
}

impl<'a, 'b, F, S> CharacterSelectionState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>> + 'static,
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

impl<'a, 'b, F, S> State<GameData<'a, 'b>> for CharacterSelectionState<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: State<GameData<'a, 'b>> + 'static,
{
    fn fixed_update(&mut self, data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);

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
