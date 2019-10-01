use amethyst::ecs::{storage::VecStorage, Component};
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};

/// Number of ticks to stay on the current frame before switching to the next frame.
#[numeric_newtype]
#[derive(Component, Debug, Default, Deserialize, Hash, Serialize)]
#[storage(VecStorage)]
pub struct Wait(pub u32);
