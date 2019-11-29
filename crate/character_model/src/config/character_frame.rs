use derive_new::new;
use object_model::config::{GameObjectFrame, ObjectFrame};
use serde::{Deserialize, Serialize};

use crate::config::CharacterInputReactions;

/// Sequence frame type for characters.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct CharacterFrame {
    /// Common object behaviour specification that can change each tick.
    #[serde(flatten)]
    pub object_frame: ObjectFrame,
    /// Sequence ID to transition to when a `ControlAction` is pressed, held, or released.
    #[serde(default)]
    pub transitions: CharacterInputReactions,
}

impl GameObjectFrame for CharacterFrame {
    fn object_frame(&self) -> &ObjectFrame {
        &self.object_frame
    }
}
