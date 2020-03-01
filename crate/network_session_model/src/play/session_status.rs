use crate::play::SessionCode;

/// Whether a network session is in play.
///
/// This is used to determine if network session systems should run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionStatus {
    /// No network session is active.
    None,
    /// Session join request has been sent, response is pending.
    JoinRequested {
        /// The session code that the request was made with.
        session_code: SessionCode,
    },
    /// A network session is active, and this client is the joiner.
    JoinEstablished,
    /// Session hosting request has been sent, response is pending.
    HostRequested,
    /// A network session is active, and this client is the host.
    HostEstablished,
}

impl Default for SessionStatus {
    fn default() -> Self {
        SessionStatus::None
    }
}
