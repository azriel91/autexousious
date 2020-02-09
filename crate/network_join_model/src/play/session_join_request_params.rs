use derive_new::new;
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use crate::play::{SessionCode, SessionDeviceName};

/// Parameters required to join a session.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct SessionJoinRequestParams {
    /// Name of the player's computer.
    pub session_device_name: SessionDeviceName,
    /// Code of the session to join.
    pub session_code: SessionCode,
}
