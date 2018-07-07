use amethyst::prelude::*;

/// `Push`es to each `State`s in reverse order of the states provided.
///
/// This implementation does not override the `Trans`ition returned by the `State` that is pushed
/// to.
#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct SchedulerState<T> {
    /// States to switch through, in reverse order.
    #[derivative(Debug = "ignore")]
    states: Vec<Box<State<T>>>,
}

impl<T> State<T> for SchedulerState<T> {
    fn update(&mut self, _data: StateData<T>) -> Trans<T> {
        if let Some(state) = self.states.pop() {
            Trans::Push(state)
        } else {
            Trans::Pop
        }
    }
}
