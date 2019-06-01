use std::iter;

use amethyst::{
    error::{format_err, ResultExt},
    input::{Axis as InputAxis, Bindings, Button},
    Error,
};
use derive_new::new;
use log::error;
use serde::{Deserialize, Serialize};

use crate::{ControlBindings, ControllerConfig, PlayerActionControl, PlayerAxisControl};

/// Structure for holding the input configuration.
#[derive(Clone, Debug, Default, Serialize, Deserialize, new)]
pub struct InputConfig {
    /// Axis control configuration.
    pub controller_configs: Vec<ControllerConfig>,
}

impl<'config> From<&'config InputConfig> for Bindings<ControlBindings> {
    fn from(input_config: &'config InputConfig) -> Bindings<ControlBindings> {
        let mut bindings = Bindings::new();

        // Axis controls
        let axis_result = input_config
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
            .fold(
                Ok(None),
                |cumulative_result, (player_axis_control, input_axis)| {
                    cumulative_result.and(
                        bindings
                            .insert_axis(player_axis_control, input_axis)
                            // kcov-ignore-start
                            .with_context(|_| {
                                Error::from_string(format!("{}", player_axis_control))
                            }),
                        // kcov-ignore-end
                    )
                },
            );

        // Action controls
        let action_result = input_config
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
            .fold(
                Ok(()),
                |cumulative_result, (player_action_control, input_button)| {
                    cumulative_result.and(
                        bindings
                            .insert_action_binding(player_action_control, iter::once(input_button))
                            // kcov-ignore-start
                            .with_context(|_| {
                                Error::from_string(format!("{}", player_action_control))
                            }),
                        // kcov-ignore-end
                    )
                },
            );

        // TODO: Bubble up result with `TryFrom`.
        // TODO: Pending <https://github.com/rust-lang/rust/issues/33417>
        if let Err(e) = &axis_result {
            error!("{}", format_err!("{}", e)); // kcov-ignore
        }
        if let Err(e) = &action_result {
            error!("{}", format_err!("{}", e)); // kcov-ignore
        }
        if axis_result.and(action_result).is_err() {
            panic!("Failed to convert `InputConfig` into `Bindings`."); // kcov-ignore
        }

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
    use crate::{
        Axis, ControlAction, ControlBindings, ControllerConfig, PlayerActionControl,
        PlayerAxisControl,
    };

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

        let bindings = Bindings::<ControlBindings>::from(&input_config);

        assert_that!(
            &bindings.axes().map(Clone::clone).collect::<Vec<_>>(),
            contains(vec![
                PlayerAxisControl::new(0, Axis::X),
                PlayerAxisControl::new(1, Axis::X)
            ])
        );
        // kcov-ignore-start
        assert_that!(
            // kcov-ignore-end
            &bindings.actions().map(Clone::clone).collect::<Vec<_>>(),
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
