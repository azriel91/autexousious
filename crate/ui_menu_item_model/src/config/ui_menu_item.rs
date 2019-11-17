use derive_new::new;
use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};
use ui_label_model::config::{UiLabel, UiSpriteLabel};

/// Specifies an object to ui_menu_item.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiMenuItem<I> {
    /// Position of the menu item.
    pub position: PositionInit,
    /// Text to display.
    pub label: UiLabel,
    /// Sprite sequence to display.
    pub sprite: UiSpriteLabel,
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

impl<I> AsRef<UiSpriteLabel> for UiMenuItem<I> {
    fn as_ref(&self) -> &UiSpriteLabel {
        &self.sprite
    }
}
