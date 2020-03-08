use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use crate::play::SessionStartRequestParams;

/// Session lobby state events.
///
/// # Examples
///
/// When read in as a command, the command string should look like the following:
///
/// * `session_lobby session_start_request --session-code ABCD`
/// * `session_lobby session_start_notify`
/// * `session_lobby back`
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[structopt(rename_all = "snake_case")]
pub enum SessionLobbyEvent {
    /// Host has requested to start the session.
    SessionStartRequest(SessionStartRequestParams),
    /// Notification from the session server to start the session.
    SessionStartNotify,
    /// Return to the previous menu.
    Back,
}
