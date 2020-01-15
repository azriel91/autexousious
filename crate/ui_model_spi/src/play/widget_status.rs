use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;

/// Active / Idle status of a UI widget.
#[derive(Clone, Copy, Component, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
#[storage(VecStorage)]
pub enum WidgetStatus {
    /// Widget is not active.
    #[derivative(Default)]
    Idle,
    /// Widget is active.
    Active,
}
