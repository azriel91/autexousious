use amethyst::input::Button;
use derive_new::new;
use sequence_model::loaded::SequenceId;

use crate::config::InputReactionAppEvents;

/// Transition to a specified sequence on device button press event.
#[derive(Clone, Debug, PartialEq, new)]
pub struct ReactionEffectButton {
    /// Device button that this transition applies to.
    pub button: Button,
    /// ID of the sequence to switch to.
    pub sequence_id: SequenceId,
    /// Events to send.
    pub events: InputReactionAppEvents,
}
