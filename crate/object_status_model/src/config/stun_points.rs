use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use serde::{Deserialize, Serialize};

/// Default stun to inflict on hit.
pub const STUN_POINTS_DEFAULT: u32 = 30;

/// Stun points of an object.
#[numeric_newtype]
#[derive(Component, Debug, Derivative, Deserialize, Hash, Serialize)]
#[derivative(Default)]
#[storage(VecStorage)]
pub struct StunPoints(#[derivative(Default(value = "STUN_POINTS_DEFAULT"))] pub u32);
