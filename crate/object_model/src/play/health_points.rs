use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};
use numeric_newtype_derive::numeric_newtype;
use specs_derive::Component;

/// Health points of an object.
#[numeric_newtype]
#[derive(Component, Debug, Derivative)]
#[storage(VecStorage)]
#[derivative(Default)]
pub struct HealthPoints(#[derivative(Default(value = "100"))] pub u32);
