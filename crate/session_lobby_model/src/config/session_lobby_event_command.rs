use serde::{Deserialize, Serialize};

/// Configuration parameters to send a `SessionLobbyEvent`.
///
/// This excludes `SessionHostEvent::SessionStartNotify` because that should be
/// sent from the session server. For testing purposes, you may still use stdin.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum SessionLobbyEventCommand {
    /// Host has requested to start the session.
    SessionStartRequest,
    /// Return to the previous menu.
    Back,
}
