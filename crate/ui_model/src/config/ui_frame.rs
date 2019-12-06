use std::convert::AsRef;

use derive_new::new;
use input_reaction_model::config::InputReactions;
use sequence_model::config::Wait;
use serde::{Deserialize, Serialize};
use sprite_model::config::{Scale, SpriteFrame, SpriteRef, SpriteSequenceName, Tint};

/// Sequence frame type for characters.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct UiFrame {
    /// Sprite rendering information.
    #[serde(flatten)]
    pub sprite_frame: SpriteFrame,
    /// Sequence ID to transition to when a `ControlAction` is pressed, held, or released.
    #[serde(default)]
    pub input_reactions: InputReactions<SpriteSequenceName>,
}

impl AsRef<Wait> for UiFrame {
    fn as_ref(&self) -> &Wait {
        &self.sprite_frame.wait
    }
}

impl AsRef<SpriteRef> for UiFrame {
    fn as_ref(&self) -> &SpriteRef {
        &self.sprite_frame.sprite
    }
}

impl AsRef<Tint> for UiFrame {
    fn as_ref(&self) -> &Tint {
        &self.sprite_frame.tint
    }
}

impl AsRef<Scale> for UiFrame {
    fn as_ref(&self) -> &Scale {
        &self.sprite_frame.scale
    }
}

impl AsRef<InputReactions<SpriteSequenceName>> for UiFrame {
    fn as_ref(&self) -> &InputReactions<SpriteSequenceName> {
        &self.input_reactions
    }
}
