use control_settings_model::config::{ControlButtonLabel, ControlButtonLabels, KeyboardSettings};
use game_input_model::InputConfig;
use log::error;
use ui_model::config::UiSequences;

use crate::ButtonToPlayerIndexMapper;

/// Generates UI items to represent the keyboard control settings.
#[derive(Debug)]
pub struct KeyboardUiGen;

impl KeyboardUiGen {
    /// Returns `UiSpriteLabel`s for the keyboard's keys, and tints player keys' sprite sequences.
    pub fn generate_full(
        keyboard_settings: &KeyboardSettings,
        input_config: &InputConfig,
        sequences: &mut UiSequences,
    ) -> ControlButtonLabels {
        let layout_positions = keyboard_settings
            .layout_positions
            .get(&keyboard_settings.layout);

        if let Some(layout_positions) = layout_positions {
            let control_button_to_player_index = ButtonToPlayerIndexMapper::map(input_config);

            let ui_sprite_labels = keyboard_settings
                .layout
                .buttons()
                .iter()
                .filter_map(|key| {
                    layout_positions
                        .get(key)
                        .cloned()
                        .map(|ui_sprite_label| (key, ui_sprite_label))
                })
                .map(|(key, mut ui_sprite_label)| {
                    // Sequence to adjust tint.
                    let ui_sequence = sequences.get_mut(&ui_sprite_label.sequence);
                    let player_index = control_button_to_player_index.get(key).copied();
                    let tint = player_index.and_then(|player_index| {
                        keyboard_settings
                            .controller_tints
                            .get(player_index)
                            .copied()
                    });

                    if let (Some(ui_sequence), Some(tint)) = (ui_sequence, tint) {
                        ui_sequence.sequence.frames.iter_mut().for_each(|ui_frame| {
                            ui_frame.sprite_frame.tint = tint;
                        });
                    }
                    ui_sprite_label.position += keyboard_settings.position;

                    ControlButtonLabel::new(ui_sprite_label, player_index)
                })
                .collect::<Vec<ControlButtonLabel>>();

            ControlButtonLabels::new(ui_sprite_labels)
        } else {
            error!(
                "Keyboard layout `{layout:?}` specified, but `\"{layout:?}\"` not found in \
                 `layout_positions`.",
                layout = keyboard_settings.layout
            );

            ControlButtonLabels::default()
        }
    }
}
