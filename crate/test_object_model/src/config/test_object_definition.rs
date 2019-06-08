use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::storage::VecStorage,
    Error,
};
use derive_new::new;
use object_model::config::{GameObjectDefinition, ObjectDefinition};
use serde::{Deserialize, Serialize};

use crate::config::TestObjectSequence;

/// Contains all of the sequences for a `TestObject`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct TestObjectDefinition {
    /// Sequences of actions this object can perform.
    #[serde(flatten)]
    pub object_definition: ObjectDefinition<TestObjectSequence>,
}

impl Asset for TestObjectDefinition {
    const NAME: &'static str = concat!(module_path!(), "::", stringify!(TestObjectDefinition));
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<TestObjectDefinition> for Result<ProcessingState<TestObjectDefinition>, Error> {
    fn from(
        character_definition: TestObjectDefinition,
    ) -> Result<ProcessingState<TestObjectDefinition>, Error> {
        Ok(ProcessingState::Loaded(character_definition))
    }
}

impl GameObjectDefinition for TestObjectDefinition {
    type GameObjectSequence = TestObjectSequence;

    fn object_definition(&self) -> &ObjectDefinition<Self::GameObjectSequence> {
        &self.object_definition
    }
}
