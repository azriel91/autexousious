use std::{convert::TryFrom, iter};

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

impl<'config> TryFrom<&'config InputConfig> for Bindings<ControlBindings> {
    type Error = Error;

    fn try_from(input_config: &'config InputConfig) -> Result<Bindings<ControlBindings>, Error> {
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

        // TODO: Extend `Error` type to support multiple causes.
        if let Err(e) = &axis_result {
            error!("{}", format_err!("{}", e)); // kcov-ignore
        }
        if let Err(e) = &action_result {
            error!("{}", format_err!("{}", e)); // kcov-ignore
        }
        axis_result.and(action_result).map(|_| bindings)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, convert::TryFrom};

    use amethyst::input::{Axis as InputAxis, BindingError, Bindings, Button};
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

        let bindings = Bindings::<ControlBindings>::try_from(&input_config)
            .expect("Failed to map `InputConfig` into `Bindings`.");

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

    #[test]
    fn try_from_returns_error_when_axis_key_bound_twice() {
        // Duplicate key A
        let controller_config_0 =
            controller_config([VirtualKeyCode::A, VirtualKeyCode::D, VirtualKeyCode::Key1]);
        let controller_config_1 =
            controller_config([VirtualKeyCode::A, VirtualKeyCode::Right, VirtualKeyCode::O]);

        let controller_configs = vec![controller_config_0, controller_config_1];
        let input_config = InputConfig::new(controller_configs);

        if let Err(error) = Bindings::<ControlBindings>::try_from(&input_config) {
            if let Some(binding_error) = error
                .source()
                .expect("Expected `BindingError` source.")
                .as_error()
                .downcast_ref::<Box<BindingError<ControlBindings>>>()
            {
                assert_eq!(
                    &Box::new(BindingError::AxisButtonAlreadyBoundToAxis(
                        PlayerAxisControl::new(0, Axis::X),
                        InputAxis::Emulated {
                            neg: Button::Key(VirtualKeyCode::A),
                            pos: Button::Key(VirtualKeyCode::D),
                        }
                    )),
                    binding_error
                );
            } else {
                // kcov-ignore-start
                panic!("Expected error type to be `Box<BindingError<ControlBindings>>`.");
                // kcov-ignore-end
            }
        } else {
            // kcov-ignore-start
            panic!("Expected to fail to map `InputConfig` into `Bindings`.");
            // kcov-ignore-end
        }
    }

    #[test]
    fn try_from_returns_error_when_action_key_bound_twice() {
        // Duplicate key A
        let controller_config_0 =
            controller_config([VirtualKeyCode::A, VirtualKeyCode::D, VirtualKeyCode::Key1]);
        let controller_config_1 = controller_config([
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::Key1,
        ]);

        let controller_configs = vec![controller_config_0, controller_config_1];
        let input_config = InputConfig::new(controller_configs);

        if let Err(error) = Bindings::<ControlBindings>::try_from(&input_config) {
            if let Some(binding_error) = error
                .source()
                .expect("Expected `BindingError` source.")
                .as_error()
                .downcast_ref::<Box<BindingError<ControlBindings>>>()
            {
                let player = 0;
                let action = ControlAction::Jump;
                assert_eq!(
                    &Box::new(BindingError::ComboAlreadyBound(PlayerActionControl::new(
                        player, action
                    ))),
                    binding_error
                );
            } else {
                // kcov-ignore-start
                panic!("Expected error type to be `Box<BindingError<ControlBindings>>`.");
                // kcov-ignore-end
            }
        } else {
            // kcov-ignore-start
            panic!("Expected to fail to map `InputConfig` into `Bindings`.");
            // kcov-ignore-end
        }
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
