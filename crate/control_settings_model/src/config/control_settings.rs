use derive_new::new;
use serde::{Deserialize, Serialize};
use ui_label_model::config::UiLabel;

use crate::config::KeyboardSettings;

/// Control Settings UI configuration.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct ControlSettings {
    /// Control settings title.
    pub title: UiLabel,
    /// Keyboard settings.
    pub keyboard: KeyboardSettings,
}

impl AsRef<UiLabel> for ControlSettings {
    fn as_ref(&self) -> &UiLabel {
        &self.title
    }
}
