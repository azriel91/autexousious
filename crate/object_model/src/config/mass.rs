use amethyst::ecs::{storage::VecStorage, Component};
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};

/// Mass of an object.
///
/// The acceleration that an object has is the product of its mass and the gravitational force.
#[numeric_newtype]
#[derive(Component, Debug, Default, Deserialize, Serialize)]
#[storage(VecStorage)]
pub struct Mass(pub f32);
