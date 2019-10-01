use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};

/// Default number of ticks to wait before another hit may occur.
const HIT_REPEAT_DELAY_DEFAULT: u32 = 10;

/// Number of ticks to wait before another hit may occur.
#[numeric_newtype]
#[derive(Component, Debug, Derivative, Deserialize, Hash, Serialize)]
#[derivative(Default)]
#[storage(VecStorage)]
pub struct HitRepeatDelay(#[derivative(Default(value = "HIT_REPEAT_DELAY_DEFAULT"))] pub u32);
