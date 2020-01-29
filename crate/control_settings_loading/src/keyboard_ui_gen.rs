use camera_model::play::CameraZoomDimensions;
use control_settings_model::{
    config::{ControlButtonLabel, ControlButtonLabels, KeyboardSettings},
    loaded::PlayerControlButtonsLabels,
};
use game_input_model::{Axis, ControlAction, ControlButton, InputConfig};
use indexmap::IndexMap;
use log::error;
use sprite_model::config::Scale;
use strum::IntoEnumIterator;
use ui_model::config::UiSequences;

use crate::{ButtonToPlayerIndexMapper, ControlButtonToButtonMapper, PcblRepositioner};

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

                    if let Some(ui_sequence) = ui_sequence {
                        ui_sequence.sequence.frames.iter_mut().for_each(|ui_frame| {
                            if let Some(tint) = tint {
                                ui_frame.sprite_frame.tint = tint;
                            }

                            if let Scale(Some(scale)) = keyboard_settings.scale {
                                ui_frame.sprite_frame.scale = keyboard_settings.scale;
                                ui_sprite_label.position *= scale;
                            }
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

    /// Returns `UiSpriteLabel`s for only control button keys, including tints.
    pub fn generate_mini(
        keyboard_settings: &KeyboardSettings,
        input_config: &InputConfig,
        camera_zoom_dimensions: CameraZoomDimensions,
        sequences: &mut UiSequences,
    ) -> Vec<PlayerControlButtonsLabels> {
        let layout_positions = keyboard_settings
            .layout_positions
            .get(&keyboard_settings.layout);

        if let Some(layout_positions) = layout_positions {
            let mut player_control_buttons_labelses =
                ControlButtonToButtonMapper::map(input_config)
                    .into_iter()
                    .enumerate()
                    .map(|(controller_id, control_buttons_to_buttons)| {
                        let (axes, actions) = control_buttons_to_buttons.into_iter().fold(
                            (
                                IndexMap::with_capacity(Axis::iter().len()),
                                IndexMap::with_capacity(ControlAction::iter().len()),
                            ),
                            |(mut axes, mut actions), (control_button, key)| {
                                let ui_sprite_label = layout_positions.get(&key).cloned();
                                if let Some(ui_sprite_label) = ui_sprite_label {
                                    // Sequence to adjust tint.
                                    let ui_sequence = sequences.get_mut(&ui_sprite_label.sequence);
                                    let tint = keyboard_settings
                                        .controller_tints
                                        .get(controller_id)
                                        .copied();

                                    if let (Some(ui_sequence), Some(tint)) = (ui_sequence, tint) {
                                        ui_sequence.sequence.frames.iter_mut().for_each(
                                            |ui_frame| {
                                                ui_frame.sprite_frame.tint = tint;
                                            },
                                        );
                                    }

                                    // Store `ControlButtonLabel` for current button.
                                    match control_button {
                                        ControlButton::Axis(control_axis) => {
                                            let control_button_label = ControlButtonLabel::new(
                                                ui_sprite_label,
                                                Some(controller_id),
                                            );
                                            axes.insert(control_axis, control_button_label);
                                        }
                                        ControlButton::Action(control_action) => {
                                            let control_button_label = ControlButtonLabel::new(
                                                ui_sprite_label,
                                                Some(controller_id),
                                            );
                                            actions.insert(control_action, control_button_label);
                                        }
                                    }
                                }

                                (axes, actions)
                            },
                        );

                        let mut player_control_buttons_labels =
                            PlayerControlButtonsLabels::new(axes, actions);

                        PcblRepositioner::reposition(&mut player_control_buttons_labels);

                        player_control_buttons_labels
                    })
                    .collect::<Vec<PlayerControlButtonsLabels>>();

            PcblRepositioner::reposition_on_screen(
                camera_zoom_dimensions,
                &mut player_control_buttons_labelses,
            );

            player_control_buttons_labelses
        } else {
            error!(
                "Keyboard layout `{layout:?}` specified, but `\"{layout:?}\"` not found in \
                 `layout_positions`.",
                layout = keyboard_settings.layout
            );

            vec![PlayerControlButtonsLabels::default(); input_config.controller_configs.len()]
        }
    }
}
