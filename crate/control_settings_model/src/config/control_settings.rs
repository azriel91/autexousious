use derive_new::new;
use serde::{Deserialize, Serialize};
use ui_label_model::config::UiLabel;

use crate::config::KeyboardLayout;

/// Control Settings UI configuration.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct ControlSettings {
    /// Control settings title.
    pub title: UiLabel,
    /// Layout of the keyboard to render.
    pub keyboard_layout: KeyboardLayout,
}

impl AsRef<UiLabel> for ControlSettings {
    fn as_ref(&self) -> &UiLabel {
        &self.title
    }
}
