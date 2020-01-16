use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;

/// Asset selection status of the `AssetSelectionHighlightMain` entity.
#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
#[storage(VecStorage)]
pub enum AshStatus {
    /// Widget is inactive.
    #[derivative(Default)]
    Inactive,
    /// Asset is being selected.
    AssetSelect,
    /// Selection has been confirmed.
    Ready,
}
