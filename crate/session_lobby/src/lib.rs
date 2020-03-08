#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! State where the game session host chooses to begin the session.

pub use crate::session_lobby_state::{
    SessionLobbyState, SessionLobbyStateBuilder, SessionLobbyStateDelegate,
};

mod session_lobby_state;
