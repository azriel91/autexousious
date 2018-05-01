use std::fmt::Debug;

use amethyst::prelude::*;

use CharacterSelection;

/// `State` where resource loading takes place.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct CharacterSelectionState<F, T>
where
    F: Fn() -> Box<T>,
    T: State + 'static,
{
    /// The `State` that follows this one.
    #[derivative(Debug(bound = "F: Debug"))]
    next_state_fn: Box<F>,
}

impl<'p, F, T> CharacterSelectionState<F, T>
where
    F: Fn() -> Box<T>,
    T: State + 'static,
{
    /// Returns a new `CharacterSelectionState`
    ///
    /// # Parameters
    ///
    /// * `next_state_fn`: Function that returns the `State` to transition to when characters have
    ///     been selected
    pub fn new(next_state_fn: Box<F>) -> Self {
        CharacterSelectionState { next_state_fn }
    }
}

impl<'p, F, T> State for CharacterSelectionState<F, T>
where
    F: Fn() -> Box<T>,
    T: State + 'static,
{
    fn fixed_update(&mut self, world: &mut World) -> Trans {
        let selected_characters = world.read_resource::<CharacterSelection>();
        if selected_characters.is_empty() {
            Trans::None
        } else {
            info!("selected_characters: `{:?}`", &*selected_characters);

            // TODO: `Trans:Push` when we have a proper character selection menu.
            Trans::Switch((self.next_state_fn)())
        }
    }
}
