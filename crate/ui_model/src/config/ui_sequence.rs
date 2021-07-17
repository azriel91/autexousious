#![allow(clippy::nonstandard_macro_braces)] // TODO: Pending https://github.com/rust-lang/rust-clippy/issues/7434

use derive_new::new;
use input_reaction_model::config::InputReactions;
use sequence_model::config::Sequence;
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteSequenceName;

use crate::config::UiFrame;

/// Sequence of `UiFrame`s.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiSequence {
    /// Object sequence for common object fields.
    #[serde(flatten)]
    pub sequence: Sequence<SpriteSequenceName, UiFrame>,
    /// Input reactions when a `ControlAction` is pressed, held, or released.
    ///
    /// This is shared by all frames in the sequence, unless overridden.
    #[serde(default)]
    pub input_reactions: Option<InputReactions<SpriteSequenceName>>,
}

impl AsRef<Sequence<SpriteSequenceName, UiFrame>> for UiSequence {
    fn as_ref(&self) -> &Sequence<SpriteSequenceName, UiFrame> {
        &self.sequence
    }
}

impl AsRef<Option<InputReactions<SpriteSequenceName>>> for UiSequence {
    fn as_ref(&self) -> &Option<InputReactions<SpriteSequenceName>> {
        &self.input_reactions
    }
}
