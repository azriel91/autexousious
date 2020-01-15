use amethyst::ecs::{storage::VecStorage, Component, Entity};
use derivative::Derivative;
use derive_new::new;

/// Siblings of a UI widget.
#[derive(Clone, Copy, Component, Debug, Derivative, PartialEq, Eq, new)]
#[derivative(Default)]
#[storage(VecStorage)]
pub struct Siblings {
    /// Previous sibling.
    pub previous: Option<Entity>,
    /// Next sibling.
    pub next: Option<Entity>,
}
