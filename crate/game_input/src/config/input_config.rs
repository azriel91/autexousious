use amethyst::input::{Axis as InputAxis, Bindings, Button};

use config::ControllerConfig;
use {PlayerActionControl, PlayerAxisControl};

/// Structure for holding the input configuration.
#[derive(Clone, Debug, Default, Serialize, Deserialize, new)]
pub struct InputConfig {
    /// Axis control configuration.
    pub controller_configs: Vec<ControllerConfig>,
}

impl<'config> From<&'config InputConfig> for Bindings<PlayerAxisControl, PlayerActionControl> {
    fn from(
        input_config: &'config InputConfig,
    ) -> Bindings<PlayerAxisControl, PlayerActionControl> {
        let mut bindings = Bindings::new();

        // Axis controls
        input_config
            .controller_configs
            .iter()
            .enumerate()
            // The enumeration index is used as the controller ID
            .flat_map(|(index, controller_config)| {
                let controller_id = index as u32;
                controller_config
                    .axes
                    .iter()
                    .map(|(&axis, input_axis)| {
                        (
                            PlayerAxisControl::new(controller_id, axis),
                            input_axis.clone(),
                        )
                    })
                    .collect::<Vec<(PlayerAxisControl, InputAxis)>>()
            })
            .for_each(|(player_axis_control, input_axis)| {
                bindings.insert_axis(player_axis_control, input_axis);
            });

        // Action controls
        input_config
            .controller_configs
            .iter()
            .enumerate()
            // The enumeration index is used as the controller ID
            .flat_map(|(index, controller_config)| {
                let controller_id = index as u32;
                controller_config
                    .actions
                    .iter()
                    .map(|(&axis, input_button)| {
                        (PlayerActionControl::new(controller_id, axis), *input_button)
                    })
                    .collect::<Vec<(PlayerActionControl, Button)>>()
            })
            .for_each(|(player_action_control, input_button)| {
                bindings.insert_action_binding(player_action_control, input_button);
            });

        bindings
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use amethyst::input::{Axis as InputAxis, Bindings, Button};
    use hamcrest::prelude::*;
    use winit::VirtualKeyCode;

    use super::InputConfig;
    use Axis;
    use ControlAction;
    use ControllerConfig;
    use PlayerActionControl;
    use PlayerAxisControl;

    #[test]
    fn bindings_from_input_config_converts_correctly() {
        let controller_config_0 =
            controller_config([VirtualKeyCode::A, VirtualKeyCode::D, VirtualKeyCode::Key1]);
        let controller_config_1 = controller_config([
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::O,
        ]);

        let controller_configs = vec![controller_config_0, controller_config_1];
        let input_config = InputConfig::new(controller_configs);

        let bindings = Bindings::<PlayerAxisControl, PlayerActionControl>::from(&input_config);

        assert_that!(
            &bindings.axes(),
            contains(vec![
                PlayerAxisControl::new(0, Axis::X),
                PlayerAxisControl::new(1, Axis::X)
            ])
        );
        assert_that!(
            &bindings.actions(),
            contains(vec![
                PlayerActionControl::new(0, ControlAction::Jump),
                PlayerActionControl::new(1, ControlAction::Jump)
            ])
        );
    }

    fn controller_config(keys: [VirtualKeyCode; 3]) -> ControllerConfig {
        let mut axes = HashMap::new();
        axes.insert(
            Axis::X,
            InputAxis::Emulated {
                neg: Button::Key(keys[0]),
                pos: Button::Key(keys[1]),
            },
        );
        let mut actions = HashMap::new();
        actions.insert(ControlAction::Jump, Button::Key(keys[2]));
        ControllerConfig::new(axes, actions)
    }
}
