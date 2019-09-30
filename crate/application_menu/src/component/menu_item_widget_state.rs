use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use strum_macros::Display;

/// Widget state of a menu item.
#[derive(Clone, Copy, Component, Debug, Derivative, Display, PartialEq, Eq)]
#[derivative(Default)]
#[storage(VecStorage)]
pub enum MenuItemWidgetState {
    /// Menu item is not highlighted.
    #[derivative(Default)]
    Idle,
    /// Menu item is highlighted.
    Active,
}
