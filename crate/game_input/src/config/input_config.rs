use amethyst::input::{Axis as InputAxis, Bindings, Button as InputButton};

use config::ControllerConfig;
use {PlayerActionControl, PlayerAxisControl};

/// Structure for holding the input configuration.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
        input_config.controller_configs.iter()
            .enumerate()
            // The enumeration index is used as the controller ID
            .flat_map(|(index, controller_config)| {
                let controller_id = index as u32;
                controller_config.axes.iter()
                    .map(|(&axis, input_axis)| (PlayerAxisControl::new(controller_id, axis), input_axis.clone()))
                    .collect::<Vec<(PlayerAxisControl, InputAxis)>>()
            })
            .for_each(|(player_axis_control, input_axis)| {bindings.insert_axis(player_axis_control, input_axis);});

        // Action controls
        input_config.controller_configs.iter()
            .enumerate()
            // The enumeration index is used as the controller ID
            .flat_map(|(index, controller_config)| {
                let controller_id = index as u32;
                controller_config.actions.iter()
                    .map(|(&axis, input_button)| (PlayerActionControl::new(controller_id, axis), input_button.clone()))
                    .collect::<Vec<(PlayerActionControl, InputButton)>>()
            })
            .for_each(|(player_action_control, input_button)| {bindings.insert_action_binding(player_action_control, input_button);});

        bindings
    }
}
