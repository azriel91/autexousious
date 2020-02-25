use amethyst::input::Button;
use derive_new::new;
use sequence_model::config::SequenceName;
use serde::{Deserialize, Serialize};

use crate::config::InputReaction;

/// Variants of how a `ButtonInputReaction` may be specified.
///
/// This is primarily to make it more ergonomic for users to specify different kinds of values in
/// configuration.
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
