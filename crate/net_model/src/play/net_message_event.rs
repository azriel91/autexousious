use derive_more::From;
use game_input_model::GameInputEvent;
use network_session_model::SessionMessageEvent;
use serde::{Deserialize, Serialize};
use session_host_model::SessionHostEvent;
use session_join_model::SessionJoinEvent;
use session_lobby_model::SessionLobbyEvent;

/// All variants of messages that can be sent over the network.
#[derive(Clone, Debug, Deserialize, From, PartialEq, Serialize)]
pub enum NetMessageEvent {
    /// `GameInputEvent` messages.
    GameInputEvent(GameInputEvent),
    /// `SessionHostEvent` messages.
    SessionHostEvent(SessionHostEvent),
    /// `SessionJoinEvent` messages.
    SessionJoinEvent(SessionJoinEvent),
    /// `SessionLobbyEvent` messages.
    SessionLobbyEvent(SessionLobbyEvent),
    /// `SessionMessageEvent` messages.
    SessionMessageEvent(SessionMessageEvent),
}
