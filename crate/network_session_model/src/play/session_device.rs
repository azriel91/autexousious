use std::str::FromStr;

use derive_new::new;
use game_input_model::{
    loaded::{PlayerController, PlayerControllers},
    play::GameInputModelError,
};
use serde::{Deserialize, Serialize};

use crate::play::{NetworkSessionModelError, SessionDeviceId, SessionDeviceName};

/// Name and ID of a session device.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct SessionDevice {
    /// Server generated ID of the session device.
    pub id: SessionDeviceId,
    /// Human readable name of the device.
    pub name: SessionDeviceName,
    /// Player controllers of the device.
    pub player_controllers: PlayerControllers,
}

impl FromStr for SessionDevice {
    type Err = NetworkSessionModelError;

    fn from_str(session_device_str: &str) -> Result<Self, NetworkSessionModelError> {
        let mut session_device_split = session_device_str.splitn(2, "::");

        // ID and name
        let id_and_name = session_device_split.next().and_then(|id_name_str| {
            let mut id_name_split = id_name_str.split(':');
            let id = id_name_split
                .next()
                .map(FromStr::from_str)
                .map(Result::ok)
                .flatten();
            let name = id_name_split
                .next()
                .map(String::from)
                .map(SessionDeviceName::from);

            if let (Some(id), Some(name)) = (id, name) {
                Some((id, name))
            } else {
                None
            }
        });

        // Player controllers
        let player_controllers = session_device_split
            .next()
            .and_then(|player_controllers_str| {
                player_controllers_str
                    .split("::")
                    .try_fold(
                        PlayerControllers::default(),
                        |mut player_controllers, player_controller_str| {
                            let player_controller =
                                PlayerController::from_str(player_controller_str)?;
                            player_controllers.push(player_controller);
                            Result::<_, GameInputModelError>::Ok(player_controllers)
                        },
                    )
                    .ok()
            });

        if let (Some((id, name)), Some(player_controllers)) = (id_and_name, player_controllers) {
            Ok(SessionDevice::new(id, name, player_controllers))
        } else {
            Err(NetworkSessionModelError::SessionDeviceParseError)
        }
    }
}
