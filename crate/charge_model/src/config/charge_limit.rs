use amethyst::ecs::{storage::VecStorage, Component};
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Limit for charge points of an object.
#[numeric_newtype]
#[derive(Component, Debug, Default, Deserialize, Hash, Serialize)]
#[storage(VecStorage)]
pub struct ChargeLimit(pub u32);
