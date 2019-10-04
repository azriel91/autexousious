use derive_new::new;
use object_model::config::{GameObjectSequence, ObjectSequence};
use serde::{Deserialize, Serialize};

use crate::config::{TestObjectFrame, TestObjectSequenceName};

/// Represents an independent action sequence of a test object.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct TestObjectSequence {
    /// Object sequence for common object fields.
    #[serde(flatten)]
    pub object_sequence: ObjectSequence<TestObjectSequenceName, TestObjectFrame>,
}

impl GameObjectSequence for TestObjectSequence {
    type SequenceName = TestObjectSequenceName;
    type GameObjectFrame = TestObjectFrame;

    fn object_sequence(&self) -> &ObjectSequence<Self::SequenceName, Self::GameObjectFrame> {
        &self.object_sequence
    }
}
