use derive_new::new;
use kinematic_model::config::PositionInit;
use sequence_model::config::SequenceNameString;
use serde::{Deserialize, Serialize};
use ui_label_model::config::UiLabel;
use ui_model_spi::config::UiSequenceName;

/// Specifies an object to ui_menu_item.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiMenuItem<I> {
    /// Position of the menu item.
    pub position: PositionInit,
    /// Text to display.
    pub label: UiLabel,
    /// `SequenceNameString` that the ui_menu_item should begin with.
    pub sequence: SequenceNameString<UiSequenceName>,
    /// Menu index this item corresponds to.
    pub index: I,
}

impl<I> AsRef<PositionInit> for UiMenuItem<I> {
    fn as_ref(&self) -> &PositionInit {
        &self.position
    }
}

impl<I> AsRef<UiLabel> for UiMenuItem<I> {
    fn as_ref(&self) -> &UiLabel {
        &self.label
    }
}
