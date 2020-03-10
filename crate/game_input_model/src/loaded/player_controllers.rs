use std::{convert::TryInto, str::FromStr};

use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::{config::PlayerInputConfigs, loaded::PlayerController, play::GameInputModelError};

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

impl FromStr for PlayerControllers {
    type Err = GameInputModelError;

    fn from_str(s: &str) -> Result<Self, GameInputModelError> {
        s.split_whitespace()
            .try_fold(
                PlayerControllers::default(),
                |mut player_controllers, player_controller_str| {
                    let player_controller = PlayerController::from_str(player_controller_str)?;
                    player_controllers.push(player_controller);
                    Ok(player_controllers)
                },
            )
            .map_err(|_: GameInputModelError| GameInputModelError::PlayerControllersParseError)
    }
}
