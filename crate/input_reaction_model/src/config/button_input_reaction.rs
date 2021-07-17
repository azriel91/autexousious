use amethyst::input::Button;
use derive_new::new;
use sequence_model::config::SequenceName;
use serde::{Deserialize, Serialize};

use crate::config::InputReaction;

/// Reaction when a device button is pressed.
///
/// This is for reacting to device (keyboard, mouse, etc.) buttons as opposed to
/// `ControlButton`s. Typically used for UIs for configuring the application,
/// rather than setting up a game.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(rename_all = "snake_case")]
pub struct ButtonInputReaction<SeqName, IRR>
where
    SeqName: SequenceName,
    IRR: Default,
{
    /// Device button that was pressed.
    pub button: Button,
    /// Variants of how an `InputReaction` may be specified.
    #[serde(flatten)]
    pub reaction: InputReaction<SeqName, IRR>,
}

impl<SeqName, IRR> AsRef<InputReaction<SeqName, IRR>> for ButtonInputReaction<SeqName, IRR>
where
    SeqName: SequenceName,
    IRR: Default,
{
    fn as_ref(&self) -> &InputReaction<SeqName, IRR> {
        &self.reaction
    }
}
