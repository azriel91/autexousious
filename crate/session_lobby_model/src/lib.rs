#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during session lobbying.

pub use crate::{session_lobby_entity::SessionLobbyEntity, session_lobby_event::SessionLobbyEvent};

pub mod config;
pub mod play;

mod session_lobby_entity;
mod session_lobby_event;
