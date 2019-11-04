use derive_new::new;

use crate::StateId;

/// Event indicating a change in the active `State`.
#[derive(Clone, Copy, Debug, PartialEq, new)]
pub struct StateIdUpdateEvent {
    /// The newly active state ID.
    pub state_id: StateId,
    /// Previously active state ID.
    pub state_id_prev: Option<StateId>,
}
