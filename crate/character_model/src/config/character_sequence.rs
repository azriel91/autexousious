use derive_new::new;
use object_model::config::{GameObjectSequence, ObjectSequence};
use sequence_model::config::Sequence;
use serde::{Deserialize, Serialize};

use crate::config::{CharacterFrame, CharacterInputReactions, CharacterSequenceName};

/// Represents an independent action sequence of a character.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
// #[serde(deny_unknown_fields)] // See <https://github.com/serde-rs/serde/issues/1547>
pub struct CharacterSequence {
    /// Object sequence for common object fields.
    #[serde(flatten)]
    pub object_sequence: ObjectSequence<CharacterSequenceName, CharacterFrame>,
    /// Sequence ID to transition to when a `ControlAction` is pressed, held, or
    /// released.
    ///
    /// This is shared by all frames in the sequence, unless overridden.
    #[serde(default)]
    pub input_reactions: Option<CharacterInputReactions>,
}

impl AsRef<Sequence<CharacterSequenceName, CharacterFrame>> for CharacterSequence {
    fn as_ref(&self) -> &Sequence<CharacterSequenceName, CharacterFrame> {
        &self.object_sequence.sequence
    }
}

impl AsRef<Option<CharacterInputReactions>> for CharacterSequence {
    fn as_ref(&self) -> &Option<CharacterInputReactions> {
        &self.input_reactions
    }
}

impl GameObjectSequence for CharacterSequence {
    type GameObjectFrame = CharacterFrame;
    type SequenceName = CharacterSequenceName;

    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceName, Self::GameObjectFrame> {
        &self.object_sequence
    }
}
