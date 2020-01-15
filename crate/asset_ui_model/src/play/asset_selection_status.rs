use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;

/// Selection status of the asset selection widget.
#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
#[storage(VecStorage)]
pub enum AssetSelectionStatus {
    /// Asset selection is deactivated.
    ///
    /// Useful for character selection when the player has not joined.
    #[derivative(Default)]
    Inactive,
    /// Asset is being selected.
    InProgress,
    /// Selection has been confirmed.
    Ready,
}
