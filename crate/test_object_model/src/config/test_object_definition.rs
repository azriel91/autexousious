use asset_derive::Asset;
use derive_new::new;
use object_model::config::{GameObjectDefinition, ObjectDefinition};
use serde::{Deserialize, Serialize};

use crate::config::TestObjectSequence;

/// Contains all of the sequences for a `TestObject`.
#[derive(Asset, Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct TestObjectDefinition {
    /// Sequences of actions this object can perform.
    #[serde(flatten)]
    pub object_definition: ObjectDefinition<TestObjectSequence>,
}

impl GameObjectDefinition for TestObjectDefinition {
    type GameObjectSequence = TestObjectSequence;

    fn object_definition(&self) -> &ObjectDefinition<Self::GameObjectSequence> {
        &self.object_definition
    }
}
