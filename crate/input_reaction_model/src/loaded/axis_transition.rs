use derive_new::new;
use game_input_model::Axis;
use sequence_model::loaded::SequenceId;

use crate::config::InputReactionAppEvents;

/// Transition to a specified fallback sequence.
#[derive(Clone, Debug, PartialEq, new)]
pub struct AxisTransition {
    /// Control button that this transition applies to.
    pub axis: Axis,
    /// ID of the sequence to switch to.
    pub sequence_id: SequenceId,
    /// Events to send.
    pub events: InputReactionAppEvents,
}
