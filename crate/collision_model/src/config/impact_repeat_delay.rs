use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Default number of ticks to wait before another impact may occur.
const IMPACT_REPEAT_DELAY_DEFAULT: u32 = 3;

/// Number of ticks to wait before another impact may occur.
#[numeric_newtype]
#[derive(Component, Debug, Derivative, Deserialize, Hash, Serialize)]
#[derivative(Default)]
#[storage(VecStorage)]
pub struct ImpactRepeatDelay(#[derivative(Default(value = "IMPACT_REPEAT_DELAY_DEFAULT"))] pub u32);
