use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use crate::play::{SessionAcceptResponse, SessionHostRequestParams, SessionRejectResponse};

/// Session host state events.
///
/// # Examples
///
/// When read in as a command, the command string should look like the following:
///
/// * `session_host session_host_request --device-name azriel`
/// * `session_host host_cancel`
/// * `session_host session_accept --session-code abcd --session-devices "1:azriel" --session-device_id 1`
/// * `session_host back`
///
/// **Note:** The `session_accept` subcommand is designed to be received from the server, so sending
/// this as a local command may cause undefined behaviour.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[structopt(rename_all = "snake_case")]
pub enum SessionHostEvent {
    /// Player entered a session code.
    SessionHostRequest(SessionHostRequestParams),
    /// Player cancelled the request to host.
    HostCancel,
    /// Server accepted the client's request.
    SessionAccept(SessionAcceptResponse),
    /// Server rejected the client's request.
    SessionReject(SessionRejectResponse),
    /// Return to the previous menu.
    Back,
}
