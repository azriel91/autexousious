use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use specs_derive::Component;

/// Charge usage subtraction variants.
#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
#[storage(VecStorage)]
pub enum ChargeUseMode {
    /// The exact number of charge points are spent.
    #[derivative(Default)]
    Exact,
    /// Subtract to the nearest multiple specified, an incomplete multiple counts.
    ///
    /// If the entity has 21 to 30 `ChargePoints`, and the charge cost is 10, it will drop to 20.
    NearestPartial,
    /// Subtract to the nearest multiple specified, an incomplete multiple does not count.
    ///
    /// If the entity has 20 to 29 `ChargePoints`, and the charge cost is 10, it will drop to 10.
    NearestWhole,
    /// All charge points are spent, regardless of charge cost.
    All,
}
