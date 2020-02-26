use serde::{Deserialize, Serialize};

/// Configuration parameters to send a `NetworkJoinEvent`.
///
/// This excludes `NetworkJoinEvent::SessionAccept` because that should be sent from the session
/// server. For testing purposes, you may still use stdin.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum NetworkJoinEventCommand {
    /// Player entered a session code.
    ///
    /// The `SessionJoinRequestParams` is specially looked up by code.
    SessionJoinRequest,
    /// Player cancelled the request to join.
    JoinCancel,
    /// Return to the previous menu.
    Back,
}
