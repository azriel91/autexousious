use amethyst::ecs::{Component, DenseVecStorage};

/// Menu item component, to be attached to relevant entities.
///
/// # Type Parameters
///
/// * `I`: Index type of the selected menu item.
#[derive(Debug)]
pub struct MenuItem<I: 'static + Send + Sync> {
    /// Index of the menu item.
    pub index: I,
}

impl<I: 'static + Send + Sync> Component for MenuItem<I> {
    type Storage = DenseVecStorage<Self>;
}
