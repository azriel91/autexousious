use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use crate::play::{SessionAcceptResponse, SessionJoinRequestParams};

/// Network join state events.
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
}
