use derive_new::new;
use object_model::config::{GameObjectSequence, ObjectSequence};
use serde::{Deserialize, Serialize};

use crate::config::{TestObjectFrame, TestObjectSequenceId};

/// Represents an independent action sequence of a test object.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct TestObjectSequence {
    /// Object sequence for common object fields.
    #[serde(flatten)]
    pub object_sequence: ObjectSequence<TestObjectSequenceId, TestObjectFrame>,
}

impl GameObjectSequence for TestObjectSequence {
    type SequenceId = TestObjectSequenceId;
    type GameObjectFrame = TestObjectFrame;

    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceId, Self::GameObjectFrame> {
        &self.object_sequence
    }
}

#[cfg(test)]
mod tests {
    use collision_model::config::{Body, Interactions};
    use object_model::config::{ObjectFrame, ObjectSequence};
    use sequence_model::config::Wait;
    use sprite_model::config::SpriteRef;
    use toml;

    use super::TestObjectSequence;
    use crate::config::TestObjectFrame;

    const SEQUENCE_WITH_FRAMES_EMPTY: &str = "frames = []";
    const SEQUENCE_WITH_CONTROL_TRANSITIONS: &str = r#"
        [[frames]]
        wait = 2
        sprite = { sheet = 0, index = 4 }
    "#;

    #[test]
    fn sequence_with_empty_frames_list_deserializes_successfully() {
        let sequence = toml::from_str::<TestObjectSequence>(SEQUENCE_WITH_FRAMES_EMPTY)
            .expect("Failed to deserialize sequence.");

        let expected = TestObjectSequence::new(ObjectSequence::new(None, vec![]));
        assert_eq!(expected, sequence);
    }

    #[test]
    fn sequence_with_control_transitions() {
        let sequence = toml::from_str::<TestObjectSequence>(SEQUENCE_WITH_CONTROL_TRANSITIONS)
            .expect("Failed to deserialize sequence.");

        let frames = vec![TestObjectFrame::new(ObjectFrame::new(
            Wait::new(2),
            SpriteRef::new(0, 4),
            Body::default(),
            Interactions::default(),
        ))];
        let expected = TestObjectSequence::new(ObjectSequence::new(None, frames));

        assert_eq!(expected, sequence);
    }
}