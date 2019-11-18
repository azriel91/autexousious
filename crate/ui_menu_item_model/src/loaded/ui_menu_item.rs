use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use derive_new::new;
use sequence_model::loaded::SequenceId;

/// Defines a UI menu item.
#[derive(Clone, Debug, ItemComponent, PartialEq, new)]
pub struct UiMenuItem<I>
where
    I: Send + Sync + 'static,
{
    /// `SequenceId` that the `UIMenuItem` should begin with.
    pub sequence_id: SequenceId,
    /// Menu index this item corresponds to.
    pub index: I,
}
