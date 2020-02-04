use derive_new::new;
use game_input_model::config::ControlAction;
use sequence_model::loaded::SequenceId;

use crate::config::InputReactionAppEvents;

/// Transition to a specified sequence on control input press event.
#[derive(Clone, Debug, PartialEq, new)]
pub struct ReactionEffectData {
    /// Control button that this transition applies to.
    pub action: ControlAction,
    /// ID of the sequence to switch to.
    pub sequence_id: SequenceId,
    /// Events to send.
    pub events: InputReactionAppEvents,
}
