use std::str::FromStr;

use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::{config::ControllerId, play::GameInputModelError};

/// Player name and their controller ID.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct PlayerController {
    /// In memory controller ID.
    pub controller_id: ControllerId,
    /// Name associated with this controller.
    pub name: String,
}

impl FromStr for PlayerController {
    type Err = GameInputModelError;

    fn from_str(s: &str) -> Result<Self, GameInputModelError> {
        let mut split = s.splitn(2, ':');
        let controller_id = split
            .next()
            .map(str::parse::<ControllerId>)
            .and_then(Result::ok);
        let name = split.next().map(ToString::to_string);

        if let (Some(controller_id), Some(name)) = (controller_id, name) {
            Ok(PlayerController::new(controller_id, name))
        } else {
            Err(GameInputModelError::PlayerControllerParseError)
        }
    }
}
