use asset_derive::Asset;
use derive_new::new;
use object_model::config::ObjectDefinition;
use serde::{Deserialize, Serialize};

use crate::config::TestObjectSequence;

/// Contains all of the sequences for a `TestObject`.
#[derive(Asset, Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct TestObjectDefinition {
    /// Sequences of actions this object can perform.
    #[serde(flatten)]
    pub object_definition: ObjectDefinition<TestObjectSequence>,
}
