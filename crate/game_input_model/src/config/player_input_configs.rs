use std::{convert::TryFrom, iter};

use amethyst::{
    error::{format_err, ResultExt},
    input::{Axis as InputAxis, Bindings, Button},
    Error,
};
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use log::error;
use serde::{Deserialize, Serialize};

use crate::{
    config::{ControlBindings, PlayerActionControl, PlayerAxisControl, PlayerInputConfig},
    play::ControllerIdOffset,
};

/// Structure for holding the input configuration.
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
pub struct PlayerInputConfigs(pub Vec<PlayerInputConfig>);

impl PlayerInputConfigs {
    /// Generates amethyst input `Bindings<ControlBindings>` for each input
    /// configuration.
    ///
    /// The `ControllerIdOffset` is used when local controllers should start
    /// with a higher index as remote controllers may use the lower indices.
    ///
    /// # Parameters
    ///
    /// * `controller_id_offset`: The offset for controller IDs.
    pub fn generate_bindings(
        &self,
        controller_id_offset: ControllerIdOffset,
    ) -> Result<Bindings<ControlBindings>, Error> {
        let mut bindings = Bindings::new();

        // Axis controls
        let axis_result = self
            .iter()
            .enumerate()
            // The enumeration index is used as the controller ID
            .flat_map(|(index, player_input_config)| {
                let controller_id = index + controller_id_offset.0;
                player_input_config
                    .controller_config
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
        let action_result = self
            .iter()
            .enumerate()
            // The enumeration index is used as the controller ID
            .flat_map(|(index, player_input_config)| {
                let controller_id = index + controller_id_offset.0;
                player_input_config
                    .controller_config
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

impl<'config> TryFrom<&'config PlayerInputConfigs> for Bindings<ControlBindings> {
    type Error = Error;

    fn try_from(
        player_input_configs: &'config PlayerInputConfigs,
    ) -> Result<Bindings<ControlBindings>, Error> {
        player_input_configs.generate_bindings(ControllerIdOffset::new(0))
    }
}
