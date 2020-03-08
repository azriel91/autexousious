use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// Error when attempting to join a session.
#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum SessionJoinError {
    /// The session code does not exist on the server.
    SessionCodeNotFound,
}
