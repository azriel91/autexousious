use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::storage::VecStorage,
    Error,
};
use derive_new::new;
use object_model::game_object;
use typename_derive::TypeName;

use crate::config::{TestObjectDefinition, TestObjectSequence, TestObjectSequenceId};

/// Represents an in-game test object that has been loaded.
///
/// Each of these fields should be a component that is attached to the test object entity.
#[game_object(TestObjectSequenceId)]
#[derive(Clone, Debug, PartialEq, TypeName, new)]
pub struct TestObject;

impl Asset for TestObject {
    const NAME: &'static str = concat!(module_path!(), "::", stringify!(TestObject));
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<TestObject> for Result<ProcessingState<TestObject>, Error> {
    fn from(test_object: TestObject) -> Result<ProcessingState<TestObject>, Error> {
        Ok(ProcessingState::Loaded(test_object))
    }
}

/// Handle to a TestObject
pub type TestObjectHandle = Handle<TestObject>;
