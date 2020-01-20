use amethyst::ecs::{storage::DenseVecStorage, Component};
use derivative::Derivative;

/// Map selection status of the `MapSelectionWidget`.
#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum MswStatus {
    /// Player has not joined.
    #[derivative(Default)]
    Inactive,
    /// Map is being selected.
    MapSelect,
    /// Selection has been confirmed.
    Ready,
}
