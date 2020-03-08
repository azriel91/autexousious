#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during network sessions.

pub use crate::session_message_event::SessionMessageEvent;

pub mod config;
pub mod play;

mod session_message_event;
