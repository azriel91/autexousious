use derive_new::new;
use game_input_model::ControlAction;

use crate::loaded::SequenceId;

/// Transition to a specified sequence on control input enabled state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct ActionHold {
    /// Control button that this transition applies to.
    pub action: ControlAction,
    /// ID of the sequence to switch to.
    pub sequence_id: SequenceId,
}
