use amethyst::{
    input::{Axis as InputAxis, Button},
    winit::VirtualKeyCode,
};
use game_input_model::{
    config::{Axis, PlayerInputConfigs},
    loaded::{ControlAxis, ControlButton},
};
use smallvec::SmallVec;

/// Creates a map of `PlayerInputConfigs` buttons to the logical control button.
#[derive(Debug)]
pub struct ControlButtonToButtonMapper;

impl ControlButtonToButtonMapper {
    /// Returns a map of `PlayerInputConfigs` buttons to the logical control button.
    ///
    /// # Parameters
    ///
    /// * `player_input_configs`: Player input configuration.
    pub fn map<'f>(
        player_input_configs: &'f PlayerInputConfigs,
    ) -> impl Iterator<Item = SmallVec<[(ControlButton, VirtualKeyCode); 8]>> + 'f {
        // TODO: Support all kinds of `amethyst::input::Button`s
        // Pending <https://github.com/amethyst/amethyst/pull/2041>.
        player_input_configs
            .controller_configs
            .values()
            .map(|controller_config| {
                let mut buttons = SmallVec::<[(ControlButton, VirtualKeyCode); 8]>::new();

                controller_config
                    .axes
                    .iter()
                    .for_each(|(axis, axis_button)| {
                        let (negative, positive) = match axis {
                            Axis::X => (ControlAxis::Left, ControlAxis::Right),
                            Axis::Z => (ControlAxis::Down, ControlAxis::Up),
                        };
                        if let InputAxis::Emulated { pos, neg } = axis_button {
                            if let Button::Key(virtual_key_code) = pos {
                                buttons.push((ControlButton::Axis(negative), *virtual_key_code));
                            }
                            if let Button::Key(virtual_key_code) = neg {
                                buttons.push((ControlButton::Axis(positive), *virtual_key_code));
                            }
                        }
                    });
                controller_config
                    .actions
                    .iter()
                    .for_each(|(control_action, action_button)| {
                        if let Button::Key(virtual_key_code) = action_button {
                            buttons
                                .push((ControlButton::Action(*control_action), *virtual_key_code));
                        }
                    });

                buttons
            })
    }
}
