use derive_new::new;
use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};
use ui_label_model::config::{UiLabel, UiSpriteLabel};
use ui_model_spi::config::WidgetStatusSequences;

use crate::config::UiTextInput;

/// Specifies a form item to fill in.
///
/// This includes the label for the item, and the input field.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiFormItem {
    /// Position of the form item.
    #[serde(default)]
    pub position: PositionInit,
    /// Text to display.
    #[serde(default)]
    pub label: UiLabel,
    /// Sprite sequence to display.
    pub sprite: UiSpriteLabel,
    /// Sequences to switch to when `WidgetStatus` changes.
    #[serde(default)]
    pub widget_status_sequences: WidgetStatusSequences,
    /// Input text field.
    #[serde(default)]
    pub input_field: UiTextInput,
}

impl AsRef<PositionInit> for UiFormItem {
    fn as_ref(&self) -> &PositionInit {
        &self.position
    }
}

impl AsRef<UiLabel> for UiFormItem {
    fn as_ref(&self) -> &UiLabel {
        &self.label
    }
}

impl AsRef<UiSpriteLabel> for UiFormItem {
    fn as_ref(&self) -> &UiSpriteLabel {
        &self.sprite
    }
}
