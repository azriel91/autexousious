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
pub struct CharacterSelectionState<'a, 'b> {
    /// The `State` that follows this one.
    #[derivative(Debug = "ignore")]
    next_state: Option<Box<State<GameData<'a, 'b>>>>,
    /// Data type used by this state and the returned state (see `StateData`).
    state_data: PhantomData<GameData<'a, 'b>>,
}

impl<'a, 'b> CharacterSelectionState<'a, 'b> {
    /// Returns a new `CharacterSelectionState`
    ///
    /// # Parameters
    ///
    /// * `next_state`: `State` to transition to when characters have been selected.
    pub fn new(next_state: Box<State<GameData<'a, 'b>>>) -> Self {
        CharacterSelectionState {
            next_state: Some(next_state),
            state_data: PhantomData,
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for CharacterSelectionState<'a, 'b> {
    fn fixed_update(&mut self, data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>> {
        let selected_characters = data.world.read_resource::<CharacterSelection>();
        if selected_characters.is_empty() {
            Trans::None
        } else {
            info!("selected_characters: `{:?}`", &*selected_characters);

            // TODO: `Trans:Push` when we have a proper character selection menu.
            Trans::Switch(self.next_state.take().unwrap())
        }
    }
}
