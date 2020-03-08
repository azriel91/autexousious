#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for the network play lobby process.

pub use crate::system::{
    SessionLobbyRequestSystem, SessionLobbyRequestSystemDesc, SessionLobbyResponseSystem,
    SessionLobbyResponseSystemDesc,
};

mod system;
