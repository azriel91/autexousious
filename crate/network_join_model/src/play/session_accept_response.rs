use derive_new::new;
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use network_session_model::play::{SessionCode, SessionDevices};

/// Response when a session join request is accepted.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct SessionAcceptResponse {
    /// Code of the session.
    #[structopt(short, long)]
    pub session_code: SessionCode,
    /// Devices already in the session.
    #[structopt(long)]
    pub session_devices: SessionDevices,
}
