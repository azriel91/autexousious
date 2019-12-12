use std::{convert::TryFrom, iter};

use amethyst::{
    error::{format_err, ResultExt},
    input::{Axis as InputAxis, Bindings, Button},
    Error,
};
use derive_new::new;
use indexmap::IndexMap;
use log::error;
use serde::{Deserialize, Serialize};

use crate::{ControlBindings, ControllerConfig, PlayerActionControl, PlayerAxisControl};

/// Structure for holding the input configuration.
#[derive(Clone, Debug, Default, Serialize, Deserialize, new)]
pub struct InputConfig {
    /// Axis control configuration.
    pub controller_configs: IndexMap<String, ControllerConfig>,
}

impl<'config> TryFrom<&'config InputConfig> for Bindings<ControlBindings> {
    type Error = Error;

    fn try_from(input_config: &'config InputConfig) -> Result<Bindings<ControlBindings>, Error> {
        let mut bindings = Bindings::new();

        // Axis controls
        let axis_result = input_config
            .controller_configs
            .values()
            .enumerate()
            // The enumeration index is used as the controller ID
            .flat_map(|(index, controller_config)| {
                let controller_id = index;
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
            .values()
            .enumerate()
            // The enumeration index is used as the controller ID
            .flat_map(|(index, controller_config)| {
                let controller_id = index;
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
