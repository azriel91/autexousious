use derive_new::new;
use game_input_model::ControlAction;

use crate::config::SequenceId;

/// Transition to a specified sequence on control input release event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct ActionRelease<SeqId>
where
    SeqId: SequenceId,
{
    /// Control button that this transition applies to.
    pub action: ControlAction,
    /// ID of the sequence to switch to.
    pub sequence_id: SeqId,
}
