use control_settings_model::config::KeyboardSettings;
use ui_label_model::config::{UiSpriteLabel, UiSpriteLabels};

/// Generates UI items to represent the keyboard control settings.
#[derive(Debug)]
pub struct KeyboardUiGen;

impl KeyboardUiGen {
    /// Returns `UiSpriteLabel`s for the keys on the specified keyboard layout.
    pub fn generate(keyboard_settings: &KeyboardSettings) -> UiSpriteLabels {
        let layout_positions = keyboard_settings
            .layout_positions
            .get(&keyboard_settings.layout)
            .unwrap_or_else(|| {
                panic!(
                    "Keyboard layout `{:?}` specified, but `\"{:?}\"` not found in \
                     `layout_positions`."
                )
            });
        let ui_sprite_labels = keyboard_settings
            .layout
            .buttons()
            .iter()
            .filter_map(|key| layout_positions.get(key).cloned())
            .map(|mut ui_sprite_label| {
                ui_sprite_label.position += keyboard_settings.position;
                ui_sprite_label
            })
            .collect::<Vec<UiSpriteLabel>>();

        UiSpriteLabels::new(ui_sprite_labels)
    }
}
