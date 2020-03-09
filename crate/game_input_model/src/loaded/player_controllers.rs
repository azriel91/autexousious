use std::convert::TryInto;

use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::{config::PlayerInputConfigs, loaded::PlayerController};

/// Player names and their controller IDs (`Vec<PlayerController>` newtype).
///
/// This includes local and remote players (if any).
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct PlayerControllers(pub Vec<PlayerController>);

impl<'c> From<&'c PlayerInputConfigs> for PlayerControllers {
    fn from(inputs: &'c PlayerInputConfigs) -> Self {
        let player_controllers = inputs
            .iter()
            .enumerate()
            .map(|(index, player_input_config)| {
                PlayerController::new(
                    (index as u32)
                        .try_into()
                        .expect("Failed to map `usize` into `u32`."),
                    player_input_config.name.clone(),
                )
            })
            .collect::<Vec<PlayerController>>();

        PlayerControllers::new(player_controllers)
    }
}
