use derive_new::new;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::play::{SessionCode, SessionDevices};

/// Session code and devices in a session.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt, new)]
pub struct Session {
    /// Code of the session.
    #[structopt(long)]
    pub session_code: SessionCode,
    /// Devices in the session.
    #[structopt(long)]
    pub session_devices: SessionDevices,
}
