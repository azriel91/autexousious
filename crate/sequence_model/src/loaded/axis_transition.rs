use derive_new::new;
use game_input_model::Axis;

use crate::config::SequenceId;

/// Transition to a specified fallback sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct AxisTransition<SeqId>
where
    SeqId: SequenceId,
{
    /// Control button that this transition applies to.
    pub axis: Axis,
    /// ID of the sequence to switch to.
    pub sequence_id: SeqId,
}
