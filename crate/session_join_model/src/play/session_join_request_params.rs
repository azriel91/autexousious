use derive_new::new;
use game_input_model::loaded::PlayerControllers;
use network_session_model::play::{SessionCode, SessionDeviceName};
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

/// Parameters required to join a session.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct SessionJoinRequestParams {
    /// Code of the session to join.
    #[structopt(long)]
    pub session_code: SessionCode,
    /// Name of the player's computer.
    #[structopt(long = "device-name")]
    pub session_device_name: SessionDeviceName,
    /// Player controllers from this session device.
    #[structopt(long)]
    pub player_controllers: PlayerControllers,
}
