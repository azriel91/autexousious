use derive_new::new;

use crate::config::SequenceId;

/// Transition to a specified fallback sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct FallbackTransition<SeqId>
where
    SeqId: SequenceId,
{
    /// ID of the sequence to switch to.
    pub sequence_id: SeqId,
}
