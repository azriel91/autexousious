use derive_new::new;
use object_model::config::{GameObjectSequence, ObjectSequence};
use serde::{Deserialize, Serialize};

use crate::config::{CharacterControlTransitions, CharacterFrame, CharacterSequenceName};

/// Represents an independent action sequence of a character.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct CharacterSequence {
    /// Object sequence for common object fields.
    #[serde(flatten)]
    pub object_sequence: ObjectSequence<CharacterSequenceName, CharacterFrame>,
    /// Sequence ID to transition to when a `ControlAction` is pressed, held, or released.
    ///
    /// This is shared by all frames in the sequence, unless overridden.
    #[serde(default)]
    pub transitions: Option<CharacterControlTransitions>,
}

impl GameObjectSequence for CharacterSequence {
    type SequenceName = CharacterSequenceName;
    type GameObjectFrame = CharacterFrame;

    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceName, Self::GameObjectFrame> {
        &self.object_sequence
    }
}
