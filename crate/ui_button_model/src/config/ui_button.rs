use derive_new::new;
use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};
use ui_label_model::config::{UiLabel, UiSpriteLabel};

/// Defines a UI widget with text, sprite, and responsive behaviour.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiButton {
    /// Position of the button.
    pub position: PositionInit,
    /// Text to display.
    pub label: UiLabel,
    /// Sprite sequence to display.
    pub sprite: UiSpriteLabel,
}

impl AsRef<PositionInit> for UiButton {
    fn as_ref(&self) -> &PositionInit {
        &self.position
    }
}

impl AsRef<UiLabel> for UiButton {
    fn as_ref(&self) -> &UiLabel {
        &self.label
    }
}

impl AsRef<UiSpriteLabel> for UiButton {
    fn as_ref(&self) -> &UiSpriteLabel {
        &self.sprite
    }
}
