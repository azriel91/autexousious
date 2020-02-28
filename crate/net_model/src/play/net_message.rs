use derive_more::From;
use serde::{Deserialize, Serialize};
use session_join_model::SessionJoinEvent;

/// All variants of messages that can be sent over the network.
#[derive(Clone, Debug, Deserialize, From, PartialEq, Serialize)]
pub enum NetMessage {
    /// `SessionJoinEvent` messages.
    SessionJoinEvent(SessionJoinEvent),
}
