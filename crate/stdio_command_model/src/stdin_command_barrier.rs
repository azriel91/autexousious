use derivative::Derivative;
use derive_new::new;
use state_registry::StateId;

/// Resource to indicate a state that the `StdinSystem` should wait before
/// sending events.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, new)]
#[derivative(Default)]
pub struct StdinCommandBarrier {
    /// Identifier of the state to ensure is running before sending the command.
    pub state_id: Option<StateId>,
}
