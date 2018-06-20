use amethyst::prelude::*;

/// Empty Amethyst State that returns `Trans::Pop` on `.update()`.
#[derive(Debug)]
pub struct EmptyState;

impl<T> State<T> for EmptyState {
    fn update(&mut self, _data: StateData<T>) -> Trans<T> {
        Trans::Pop
    }
}
