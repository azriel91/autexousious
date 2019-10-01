use std::fmt::Debug;

use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_new::new;

/// Menu item component, to be attached to relevant entities.
///
/// # Type Parameters
///
/// * `I`: Index type of the selected menu item.
#[derive(Clone, Component, Debug, PartialEq, new)]
pub struct MenuItem<I>
where
    I: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
{
    /// Index of the menu item.
    pub index: I,
}
