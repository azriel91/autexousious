use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};

/// Default number of ticks to wait before transitioning to the next frame.
pub const WAIT_DEFAULT: u32 = 2;

/// Number of ticks to stay on the current frame before switching to the next
/// frame.
#[numeric_newtype]
#[derive(Component, Debug, Derivative, Deserialize, Serialize)]
#[derivative(Default)]
#[storage(VecStorage)]
pub struct Wait(#[derivative(Default(value = "WAIT_DEFAULT"))] pub u32);
