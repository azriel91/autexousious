use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use crate::play::{SessionAcceptResponse, SessionJoinRequestParams};

/// Network join state events.
///
/// # Examples
///
/// When read in as a command, the command string should look like the following:
///
/// * `network_join session_join_request --device-name azriel --session-code abcd`
/// * `network_join join_cancel`
/// * `network_join session_accept --session-code abcd --session-devices "1:azriel 2:byron 3:carlo"`
/// * `network_join back`
///
/// **Note:** The `session_accept` subcommand is designed to be received from the server, so sending
/// this as a local command may cause undefined behaviour.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[structopt(rename_all = "snake_case")]
pub enum NetworkJoinEvent {
    /// Player entered a session code.
    SessionJoinRequest(SessionJoinRequestParams),
    /// Player cancelled the request to join.
    JoinCancel,
    /// Server accepted the client's request.
    SessionAccept(SessionAcceptResponse),
    /// Return to the previous menu.
    Back,
}
