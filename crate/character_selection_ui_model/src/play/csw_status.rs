use amethyst::ecs::{storage::DenseVecStorage, Component};
use derivative::Derivative;

/// Character selection status of the `CharacterSelectionWidget`.
#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum CswStatus {
    /// Player has not joined.
    #[derivative(Default)]
    Inactive,
    /// Character is being selected.
    CharacterSelect,
    /// Selection has been confirmed.
    Ready,
}
