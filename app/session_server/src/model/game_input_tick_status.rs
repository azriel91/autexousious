/// Whether a client has sent the `SessionMessageEvent::GameInputTick` message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameInputTickStatus {
    /// Client has not sent `SessionMessageEvent::GameInputTick` this tick.
    Pending,
    /// Client has already sent `SessionMessageEvent::GameInputTick` this tick.
    Received,
}

impl Default for GameInputTickStatus {
    fn default() -> Self {
        Self::Pending
    }
}
