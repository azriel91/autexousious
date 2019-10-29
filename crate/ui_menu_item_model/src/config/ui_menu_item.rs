use derive_new::new;
use sequence_model::config::SequenceNameString;
use serde::{Deserialize, Serialize};

use crate::config::UiMenuItemSequenceName;

/// Specifies an object to ui_menu_item.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiMenuItem<I> {
    /// Menu index this item corresponds to.
    pub index: I,
    /// Text to display.
    #[serde(default)]
    pub text: String,
    /// `SequenceNameString` that the ui_menu_item should begin with.
    pub sequence: SequenceNameString<UiMenuItemSequenceName>,
}
