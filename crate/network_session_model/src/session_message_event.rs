use serde::{Deserialize, Serialize};

use crate::play::SessionDeviceJoin;

/// Session message events.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum SessionMessageEvent {
    /// An additional device joined the session.
    SessionDeviceJoin(SessionDeviceJoin),
}
