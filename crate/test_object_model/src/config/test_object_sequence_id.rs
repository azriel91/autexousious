use amethyst::ecs::{
    storage::{FlaggedStorage, VecStorage},
    Component,
};
use derivative::Derivative;
use sequence_model::config::SequenceId;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, IntoStaticStr};

/// `TestObject` Sequence IDs.
#[derive(
    Clone,
    Copy,
    Debug,
    Derivative,
    Deserialize,
    Display,
    EnumString,
    IntoStaticStr,
    PartialEq,
    Eq,
    Hash,
    Serialize,
)]
#[derivative(Default)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TestObjectSequenceId {
    /// Default sequence.
    #[derivative(Default)]
    Zero,
    /// Sequence one.
    One,
}

/// Not every entity will have this, but since this is probably a `u8`, we don't need an indirection
/// table.
impl Component for TestObjectSequenceId {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl SequenceId for TestObjectSequenceId {}
