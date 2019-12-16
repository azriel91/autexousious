use derive_new::new;
use game_input_model::ControllerId;
use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};
use ui_label_model::config::UiSpriteLabel;

/// Specifies an object to ui_menu_item.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct ControlButtonLabel {
    /// Sprite sequence to display.
    pub sprite: UiSpriteLabel,
    /// Controller ID that this label is linked to, if any.
    pub controller_id: Option<ControllerId>,
}

impl AsRef<UiSpriteLabel> for ControlButtonLabel {
    fn as_ref(&self) -> &UiSpriteLabel {
        &self.sprite
    }
}

impl AsRef<PositionInit> for ControlButtonLabel {
    fn as_ref(&self) -> &PositionInit {
        &self.sprite.position
    }
}

impl AsRef<Option<ControllerId>> for ControlButtonLabel {
    fn as_ref(&self) -> &Option<ControllerId> {
        &self.controller_id
    }
}
