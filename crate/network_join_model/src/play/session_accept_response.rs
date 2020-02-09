use derive_new::new;
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use crate::play::SessionDevices;

/// Response when a session join request is accepted.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct SessionAcceptResponse {
    /// Name of the player's device.
    pub session_devices: SessionDevices,
    /// Code of the session to join.
    pub session_code: String,
}
