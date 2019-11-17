use derive_new::new;
use sequence_model::loaded::SequenceId;

/// Defines a UI menu item.
#[derive(Clone, Debug, PartialEq, new)]
pub struct UiMenuItem<I> {
    /// `SequenceId` that the `UIMenuItem` should begin with.
    pub sequence_id: SequenceId,
    /// Menu index this item corresponds to.
    pub index: I,
}
