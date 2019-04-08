use derive_new::new;
use object_model::config::{GameObjectSequence, ObjectSequence};
use serde::{Deserialize, Serialize};

use crate::config::character_sequence_id::CharacterSequenceId;

/// Represents an independent action sequence of a character.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct CharacterSequence {
    /// Object sequence for common object fields.
    #[serde(flatten)]
    pub object_sequence: ObjectSequence<CharacterSequenceId>,
}

impl GameObjectSequence for CharacterSequence {
    type SequenceId = CharacterSequenceId;

    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceId> {
        &self.object_sequence
    }
}

#[cfg(test)]
mod tests {
    use object_model::config::ObjectSequence;
    use toml;

    use super::CharacterSequence;

    const SEQUENCE_WITH_FRAMES_EMPTY: &str = "frames = []";

    #[test]
    fn sequence_with_empty_frames_list_deserializes_successfully() {
        let sequence = toml::from_str::<CharacterSequence>(SEQUENCE_WITH_FRAMES_EMPTY)
            .expect("Failed to deserialize sequence.");

        let expected = CharacterSequence::new(ObjectSequence::new(None, vec![]));
        assert_eq!(expected, sequence);
    }
}
