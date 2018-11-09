use derive_more::{Add, AddAssign, Display, From};

/// Health points of an object.
#[derive(Add, AddAssign, Clone, Copy, Debug, Derivative, Display, From, PartialEq, Eq)]
#[derivative(Default)]
pub struct HealthPoints(#[derivative(Default(value = "100"))] pub u32);
