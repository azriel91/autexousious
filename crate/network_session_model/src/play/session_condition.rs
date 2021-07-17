/// Conditions that must be satisfied for a network session game to proceed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionCondition {
    /// Waiting for the `SessionMessageEvent::GameInputTick` message to be
    /// received from the server.
    PendingGameInputTick,
    /// There is no pending messages for the session to proceed.
    Ready,
}

impl Default for SessionCondition {
    fn default() -> Self {
        Self::Ready
    }
}
