use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::KeyboardLayout;

/// Control Settings UI configuration.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct ControlSettings {
    /// Layout of the keyboard to render.
    pub keyboard_layout: KeyboardLayout,
}
