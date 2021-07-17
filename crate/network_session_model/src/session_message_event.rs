use serde::{Deserialize, Serialize};

use crate::play::SessionDeviceJoin;

/// Session message events.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum SessionMessageEvent {
    /// Indicates all `GameInputEvent`s for this tick have been sent.
    ///
    /// The session server waits for this message from all game clients before
    /// sending all `GameInputEvent`s to clients.
    ///
    /// Clients waits for this message from the game server before ticking the
    /// game.
    GameInputTick,
    /// An additional device joined the session.
    SessionDeviceJoin(SessionDeviceJoin),
}
