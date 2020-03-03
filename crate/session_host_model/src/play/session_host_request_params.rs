use derive_new::new;
use network_session_model::play::SessionDeviceName;
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

/// Parameters required to host a session.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct SessionHostRequestParams {
    /// Name of the player's computer.
    #[structopt(long = "device-name")]
    pub session_device_name: SessionDeviceName,
}
