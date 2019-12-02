use derive_new::new;
use game_input_model::Axis;
use sequence_model::loaded::SequenceId;

/// Transition to a specified fallback sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct AxisTransition {
    /// Control button that this transition applies to.
    pub axis: Axis,
    /// ID of the sequence to switch to.
    pub sequence_id: SequenceId,
}
