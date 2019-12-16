use amethyst::ecs::{storage::DenseVecStorage, Component};
use derivative::Derivative;
use strum_macros::Display;

/// Map selection state of the `MapSelectionWidget`.
#[derive(Clone, Component, Copy, Debug, Derivative, Display, PartialEq, Eq)]
#[derivative(Default)]
pub enum MapSelectionWidgetState {
    /// Map is being selected.
    #[derivative(Default)]
    MapSelect,
    /// Selection has been confirmed.
    Ready,
}
