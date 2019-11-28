use std::collections::HashMap;

use amethyst::{
    input::{Axis, Button},
    winit::VirtualKeyCode,
};
use control_settings_model::config::KeyboardSettings;
use game_input_model::InputConfig;
use smallvec::SmallVec;
use ui_label_model::config::{UiSpriteLabel, UiSpriteLabels};
use ui_model::config::UiSequences;

/// Generates UI items to represent the keyboard control settings.
#[derive(Debug)]
pub struct KeyboardUiGen;

impl KeyboardUiGen {
    /// Returns `UiSpriteLabel`s for the keyboard's keys, and tints player keys' sprite sequences.
    pub fn generate(
        keyboard_settings: &KeyboardSettings,
        input_config: &InputConfig,
        sequences: &mut UiSequences,
    ) -> UiSpriteLabels {
        let layout_positions = keyboard_settings
            .layout_positions
            .get(&keyboard_settings.layout)
            .unwrap_or_else(|| {
                panic!(
                    "Keyboard layout `{layout:?}` specified, but `\"{layout:?}\"` not found in \
                     `layout_positions`.",
                    layout = keyboard_settings.layout
                )
            });

        // TODO: Support all kinds of `amethyst::input::Button`s
        // Pending <https://github.com/amethyst/amethyst/pull/2041>.
        let control_button_to_player_index = input_config
            .controller_configs
            .values()
            .enumerate()
            .flat_map(|(index, controller_config)| {
                let mut buttons = SmallVec::<[VirtualKeyCode; 8]>::new();

                controller_config.axes.values().for_each(|axis| {
                    if let Axis::Emulated { pos, neg } = axis {
                        if let Button::Key(virtual_key_code) = pos {
                            buttons.push(*virtual_key_code);
                        }
                        if let Button::Key(virtual_key_code) = neg {
                            buttons.push(*virtual_key_code);
                        }
                    }
                });
                controller_config.actions.values().for_each(|button| {
                    if let Button::Key(virtual_key_code) = button {
                        buttons.push(*virtual_key_code);
                    }
                });

                buttons.into_iter().map(move |button| (button, index))
            })
            .collect::<HashMap<VirtualKeyCode, usize>>();

        let ui_sprite_labels = keyboard_settings
            .layout
            .buttons()
            .iter()
            .filter_map(|key| {
                layout_positions
                    .get(key)
                    .cloned()
                    .map(|ui_sprite_position| (key, ui_sprite_position))
            })
            .map(|(key, mut ui_sprite_label)| {
                // Sequence to adjust tint.
                let sequence = sequences.get_mut(&ui_sprite_label.sequence);
                let tint_index = control_button_to_player_index.get(key).copied();
                let tint = tint_index.and_then(|tint_index| {
                    keyboard_settings.controller_tints.get(tint_index).copied()
                });

                if let (Some(sequence), Some(tint)) = (sequence, tint) {
                    sequence.frames.iter_mut().for_each(|sprite_frame| {
                        sprite_frame.tint = tint;
                    });
                }
                ui_sprite_label.position += keyboard_settings.position;
                ui_sprite_label
            })
            .collect::<Vec<UiSpriteLabel>>();

        UiSpriteLabels::new(ui_sprite_labels)
    }
}
