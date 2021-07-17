use amethyst::ecs::{storage::VecStorage, Component};
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;

/// Number of game objects that this object has hit.
///
/// This component is needed to track number of objects hit during processing of
/// `HitEvent`s in the `HitDetectionSystem`.
#[numeric_newtype]
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct HitObjectCount(pub u32);
