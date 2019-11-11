use derive_new::new;
use kinematic_model::config::PositionInit;
use sequence_model::config::SequenceNameString;
use serde::{Deserialize, Serialize};
use ui_model_spi::config::UiSequenceName;

/// Specifies an object to ui_menu_item.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiMenuItem<I> {
    /// Menu index this item corresponds to.
    pub index: I,
    /// Text to display.
    #[serde(default)]
    pub text: String,
    /// Position of the menu item.
    pub position: PositionInit,
    /// `SequenceNameString` that the ui_menu_item should begin with.
    pub sequence: SequenceNameString<UiSequenceName>,
}

impl<I> AsRef<PositionInit> for UiMenuItem<I> {
    fn as_ref(&self) -> &PositionInit {
        &self.position
    }
}
