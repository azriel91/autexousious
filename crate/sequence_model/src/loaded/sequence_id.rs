use amethyst::ecs::{
    storage::{FlaggedStorage, VecStorage},
    Component,
};
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};

/// Sequence ID of an object.
#[numeric_newtype]
#[derive(Debug, Default, Deserialize, Hash, Serialize)]
pub struct SequenceId(pub usize);

/// Not every entity will have this, but since this is probably a `u8`, we don't need an indirection
/// table.
impl Component for SequenceId {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
