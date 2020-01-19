use amethyst::ecs::{storage::VecStorage, Component, Entity};
use derivative::Derivative;
use derive_new::new;

/// Siblings of a UI widget above and below.
#[derive(Clone, Copy, Component, Debug, Derivative, PartialEq, Eq, new)]
#[derivative(Default)]
#[storage(VecStorage)]
pub struct SiblingsVertical {
    /// Widget above this one.
    pub up: Option<Entity>,
    /// Widget below this one.
    pub down: Option<Entity>,
}
