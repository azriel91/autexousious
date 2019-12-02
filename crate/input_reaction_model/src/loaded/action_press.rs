use derive_new::new;
use game_input_model::ControlAction;
use sequence_model::loaded::SequenceId;

/// Transition to a specified sequence on control input press event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct ActionPress {
    /// Control button that this transition applies to.
    pub action: ControlAction,
    /// ID of the sequence to switch to.
    pub sequence_id: SequenceId,
}
