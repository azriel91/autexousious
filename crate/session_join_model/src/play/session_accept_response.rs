use derive_new::new;
use network_session_model::play::{SessionCode, SessionDeviceId, SessionDevices};
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

/// Response when a session join request is accepted.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct SessionAcceptResponse {
    /// Code of the session.
    #[structopt(short, long)]
    pub session_code: SessionCode,
    /// ID that the server generated for the session joiner.
    #[structopt(short, long)]
    pub session_device_id: SessionDeviceId,
    /// Devices in the session.
    ///
    /// This includes the session joiner's device.
    #[structopt(long)]
    pub session_devices: SessionDevices,
}
