use derive_new::new;

use crate::loaded::SequenceId;

/// Transition to a specified fallback sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct FallbackTransition {
    /// ID of the sequence to switch to.
    pub sequence_id: SequenceId,
}
