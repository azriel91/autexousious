use derive_new::new;
use object_model::config::{GameObjectSequence, ObjectSequence};
use sequence_model::config::Sequence;
use serde::{Deserialize, Serialize};

use crate::config::{TestObjectFrame, TestObjectSequenceName};

/// Represents an independent action sequence of a test object.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
// #[serde(deny_unknown_fields)] // See <https://github.com/serde-rs/serde/issues/1547>
pub struct TestObjectSequence {
    /// Object sequence for common object fields.
    #[serde(flatten)]
    pub object_sequence: ObjectSequence<TestObjectSequenceName, TestObjectFrame>,
}

impl AsRef<Sequence<TestObjectSequenceName, TestObjectFrame>> for TestObjectSequence {
    fn as_ref(&self) -> &Sequence<TestObjectSequenceName, TestObjectFrame> {
        &self.object_sequence.sequence
    }
}

impl GameObjectSequence for TestObjectSequence {
    type GameObjectFrame = TestObjectFrame;
    type SequenceName = TestObjectSequenceName;

    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceName, Self::GameObjectFrame> {
        &self.object_sequence
    }
}
