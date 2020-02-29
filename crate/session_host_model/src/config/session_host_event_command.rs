use serde::{Deserialize, Serialize};

/// Configuration parameters to send a `SessionHostEvent`.
///
/// This excludes `SessionHostEvent::SessionAccept` because that should be sent from the session
/// server. For testing purposes, you may still use stdin.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum SessionHostEventCommand {
    /// Player requested to host a session.
    SessionHostRequest,
    /// Player cancelled the request to host.
    HostCancel,
    /// Return to the previous menu.
    Back,
}
