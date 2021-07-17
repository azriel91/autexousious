use derive_new::new;
use object_model::config::{GameObjectFrame, ObjectFrame};
use sequence_model::config::Wait;
use serde::{Deserialize, Serialize};

use crate::config::CharacterInputReactions;

/// Sequence frame type for characters.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct CharacterFrame {
    /// Common object behaviour specification that can change each tick.
    #[serde(flatten)]
    pub object_frame: ObjectFrame,
    /// Sequence ID to transition to when a `ControlAction` is pressed, held, or
    /// released.
    #[serde(default)]
    pub input_reactions: CharacterInputReactions,
}

impl AsRef<Wait> for CharacterFrame {
    fn as_ref(&self) -> &Wait {
        &self.object_frame.wait
    }
}

impl AsRef<CharacterInputReactions> for CharacterFrame {
    fn as_ref(&self) -> &CharacterInputReactions {
        &self.input_reactions
    }
}

impl GameObjectFrame for CharacterFrame {
    fn object_frame(&self) -> &ObjectFrame {
        &self.object_frame
    }
}
