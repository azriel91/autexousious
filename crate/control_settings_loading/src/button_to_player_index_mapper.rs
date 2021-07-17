use std::collections::HashMap;

use amethyst::{
    input::{Axis, Button},
    winit::event::VirtualKeyCode,
};
use game_input_model::config::{ControllerId, PlayerInputConfigs};
use smallvec::SmallVec;

/// Creates a map of `PlayerInputConfigs` buttons to the player index that uses
/// the button.
#[derive(Debug)]
pub struct ButtonToPlayerIndexMapper;

impl ButtonToPlayerIndexMapper {
    /// Returns a map of `PlayerInputConfigs` buttons to the player index that
    /// uses the button.
    ///
    /// # Parameters
    ///
    /// * `player_input_configs`: Player input configuration.
    pub fn map(player_input_configs: &PlayerInputConfigs) -> HashMap<VirtualKeyCode, ControllerId> {
        // TODO: Support all kinds of `amethyst::input::Button`s
        // Pending <https://github.com/amethyst/amethyst/pull/2041>.
        player_input_configs
            .iter()
            .enumerate()
            .flat_map(|(index, player_input_config)| {
                let mut buttons = SmallVec::<[VirtualKeyCode; 8]>::new();

                player_input_config
                    .controller_config
                    .axes
                    .values()
                    .for_each(|axis| {
                        if let Axis::Emulated { pos, neg } = axis {
                            if let Button::Key(virtual_key_code) = pos {
                                buttons.push(*virtual_key_code);
                            }
                            if let Button::Key(virtual_key_code) = neg {
                                buttons.push(*virtual_key_code);
                            }
                        }
                    });
                player_input_config
                    .controller_config
                    .actions
                    .values()
                    .for_each(|button| {
                        if let Button::Key(virtual_key_code) = button {
                            buttons.push(*virtual_key_code);
                        }
                    });

                buttons.into_iter().map(move |button| (button, index))
            })
            .collect::<HashMap<VirtualKeyCode, ControllerId>>()
    }
}
