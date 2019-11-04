use derive_new::new;
use sequence_model::loaded::SequenceId;

/// Defines a UI menu item.
#[derive(Clone, Debug, PartialEq, new)]
pub struct UiMenuItem<I> {
    /// Menu index this item corresponds to.
    pub index: I,
    /// Text to display.
    pub text: String,
    /// `SequenceId` that the ui_menu_itemed object should begin with.
    pub sequence_id: SequenceId,
}
