use derive_new::new;
use object_model::config::{GameObjectFrame, ObjectFrame};
use sequence_model::config::ControlActionTransitions;
use serde::{Deserialize, Serialize};

use crate::config::CharacterSequenceId;

/// Sequence frame type for characters.
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct CharacterFrame {
    /// Common object behaviour specification that can change each tick.
    #[serde(flatten)]
    pub object_frame: ObjectFrame,
    /// Sequence ID to transition to when a `ControlAction` is pressed.
    #[serde(default)]
    pub transitions: ControlActionTransitions<CharacterSequenceId>,
}

impl GameObjectFrame for CharacterFrame {
    fn object_frame(&self) -> &ObjectFrame {
        &self.object_frame
    }
}
