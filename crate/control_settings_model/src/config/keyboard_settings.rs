use std::collections::HashMap;

use amethyst::winit::VirtualKeyCode;
use derive_new::new;
use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};
use ui_label_model::config::UiSpriteLabel;

use crate::config::KeyboardLayout;

/// Keyboard settings.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct KeyboardSettings {
    /// Position of the keyboard on screen.
    pub position: PositionInit,
    /// Layout of the keyboard to render.
    pub layout: KeyboardLayout,
    /// Positions of keys for each layout.
    pub layout_positions: HashMap<KeyboardLayout, HashMap<VirtualKeyCode, UiSpriteLabel>>,
}
