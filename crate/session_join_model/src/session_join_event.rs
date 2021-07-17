use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use crate::play::{SessionAcceptResponse, SessionJoinRequestParams, SessionRejectResponse};

/// Session join state events.
///
/// # Examples
///
/// When read in as a command, the command string should look like the
/// following:
///
/// * `session_join session_join_request --session-code abcd --device-name
///   azriel --player-controllers "0:azriel 1:friend_a`
/// * `session_join join_cancel`
/// * `session_join session_accept --session-code abcd --session-devices
///   "0:azriel::0:azriel::1:friend_a 1:byron::0:friend_b 2:carlo::0:friend_c"
///   --session-device-id 2`
/// * `session_join back`
///
/// **Note:** The `session_accept` subcommand is designed to be received from
/// the server, so sending this as a local command may cause undefined
/// behaviour.
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
