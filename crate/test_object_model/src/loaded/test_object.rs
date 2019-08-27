use asset_derive::Asset;
use derive_new::new;
use object_model::game_object;
use typename_derive::TypeName;

use crate::config::{TestObjectDefinition, TestObjectSequence, TestObjectSequenceName};

/// Represents an in-game test object that has been loaded.
///
/// Each of these fields should be a component that is attached to the test object entity.
#[game_object]
#[derive(Asset, Clone, Debug, PartialEq, TypeName, new)]
pub struct TestObject;
