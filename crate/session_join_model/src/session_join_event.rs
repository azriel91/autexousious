use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use crate::play::{SessionAcceptResponse, SessionJoinRequestParams, SessionRejectResponse};

/// Session join state events.
///
/// # Examples
///
/// When read in as a command, the command string should look like the following:
///
/// * `session_join session_join_request --device-name azriel --session-code abcd`
/// * `session_join join_cancel`
/// * `session_join session_accept --session-code abcd --session-devices "1:azriel 2:byron 3:carlo" --session-device-id 1`
/// * `session_join back`
///
/// **Note:** The `session_accept` subcommand is designed to be received from the server, so sending
/// this as a local command may cause undefined behaviour.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[structopt(rename_all = "snake_case")]
pub enum SessionJoinEvent {
    /// Player entered a session code.
    SessionJoinRequest(SessionJoinRequestParams),
    /// Player cancelled the request to join.
    JoinCancel,
    /// Server accepted the client's request.
    SessionAccept(SessionAcceptResponse),
    /// Server rejected the client's request.
    SessionReject(SessionRejectResponse),
    /// Return to the previous menu.
    Back,
}
